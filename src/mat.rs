use crate::{
    MatArray, MatFile, MatFileRead, MatFileWrite, MatType, MatioError, MayBeFrom, MayBeInto, Result,
};
use std::{ffi::CStr, marker::PhantomData, ptr, slice::from_raw_parts};

/// Matlab variable
pub struct Mat<'a> {
    pub(crate) name: String,
    pub(crate) matvar_t: *mut ffi::matvar_t,
    pub(crate) fields: Option<Vec<Mat<'a>>>,
    pub(crate) marker: PhantomData<&'a ffi::matvar_t>,
}
impl<'a> Drop for Mat<'a> {
    fn drop(&mut self) {
        if let Some(mut fields) = self.fields.take() {
            fields.iter_mut().for_each(|mat| {
                mat.matvar_t = ptr::null_mut();
            })
        }
        unsafe {
            ffi::Mat_VarFree(self.matvar_t);
        }
    }
}
impl<'a> MatFile<'a> {
    /// Read from a [MatFile] the Matlab [Mat] variable `name`
    pub fn read<S: Into<String>>(&self, name: S) -> Result<Mat<'a>> {
        let c_name = std::ffi::CString::new(name.into())?;
        let matvar_t = unsafe { ffi::Mat_VarRead(self.mat_t, c_name.as_ptr()) };
        if matvar_t.is_null() {
            Err(MatioError::MatVarRead(c_name.to_str().unwrap().to_string()))
        } else {
            Mat::from_ptr(c_name.to_str()?, matvar_t)
        }
    }
    /// Write to a [MatFile] the Matlab [Mat] variable `name`
    pub fn write(&self, var: Mat<'a>) -> &Self {
        unsafe {
            ffi::Mat_VarWrite(
                self.mat_t,
                var.matvar_t,
                ffi::matio_compression_MAT_COMPRESSION_NONE,
            );
        }
        self
    }
}
impl<'a> MatFileRead<'a> {
    /// Read from a [MatFileRead]er the Matlab [Mat] variable `name`
    ///
    /// Reading a scalar Matlab variable: a = π
    /// ```
    /// use matio_rs::MatFile;
    /// # let file = tempfile::NamedTempFile::new().unwrap();
    /// # let data_path = file.path();
    /// # let mat_file = MatFile::save(&data_path)?.var("a", std::f64::consts::PI)?;
    /// let a: f64 = MatFile::load(data_path)?.var("a")?;
    /// println!("{a:?}");
    /// # Ok::<(), matio_rs::MatioError>(())
    /// ```
    ///
    /// Reading a Matlab vector: b = [3.0, 1.0, 4.0, 1.0, 6.0]
    /// ```
    /// use matio_rs::MatFile;
    /// # let file = tempfile::NamedTempFile::new().unwrap();
    /// # let data_path = file.path();
    /// # let mat_file = MatFile::save(&data_path)?.var("b", vec![3.0, 1.0, 4.0, 1.0, 6.0])?;
    /// let b: Vec<f64> = MatFile::load(data_path)?.var("b")?;
    /// println!("{b:?}");
    /// # Ok::<(), matio_rs::MatioError>(())
    /// ```
    pub fn var<S: Into<String>, T>(&self, name: S) -> Result<T>
    where
        Mat<'a>: MayBeInto<T>,
    {
        self.read(name).and_then(|mat| mat.maybe_into())
    }
}
impl<'a> MatFileWrite<'a> {
    /// Write to a [MatFileWrite]r the Matlab [Mat] variable `name`
    ///
    /// Saving to a mat file
    /// ```
    /// use matio_rs::MatFile;
    /// # let file = tempfile::NamedTempFile::new().unwrap();
    /// # let data_path = file.path();
    /// let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
    /// MatFile::save(data_path)?
    /// .var("a", 2f64.sqrt())?
    /// .var("b", &b)?;
    /// # Ok::<(), matio_rs::MatioError>(())
    /// ```
    pub fn var<S: Into<String>, T>(&self, name: S, data: T) -> Result<&Self>
    where
        Mat<'a>: MayBeFrom<T>,
    {
        let mat: Mat<'a> = MayBeFrom::<T>::maybe_from(name, data)?;
        self.write(mat);
        Ok(self)
    }
    /// Write to a [MatFileWrite]r the Matlab [Mat] variable `name` as a N-dimensition array [MatArray]
    ///
    /// The data is aligned according to and in the order of the dimension vector dims
    pub fn array<S: Into<String>, T>(&self, name: S, data: &'a [T], dims: Vec<u64>) -> Result<&Self>
    where
        Mat<'a>: MayBeFrom<MatArray<'a, T>>,
    {
        let mat_array = MatArray::new(data, dims);
        self.var(name, mat_array)?;
        Ok(self)
    }
}
impl<'a> Mat<'a> {
    /// Returns the rank (# of dimensions) of the Matlab variable
    pub fn rank(&self) -> usize {
        unsafe { (*self.matvar_t).rank as usize }
    }
    /// Returns the dimensions of the Matlab variable
    pub fn dims(&self) -> Vec<usize> {
        let rank = self.rank();
        let mut dims: Vec<usize> = Vec::with_capacity(rank);
        unsafe {
            ptr::copy((*self.matvar_t).dims, dims.as_mut_ptr(), rank);
            dims.set_len(rank);
        };
        dims
    }
    /// Returns the number of elements of the Matlab variable
    pub fn len(&self) -> usize {
        self.dims().into_iter().product::<usize>() as usize
    }
    pub(crate) fn mat_type(&self) -> Option<MatType> {
        MatType::from_ptr(self.matvar_t)
    }
    pub(crate) fn from_ptr<S: Into<String>>(name: S, ptr: *mut ffi::matvar_t) -> Result<Self> {
        if let Some(MatType::STRUCT) = MatType::from_ptr(ptr) {
            let rank = unsafe { (*ptr).rank as usize };
            let mut dims: Vec<usize> = Vec::with_capacity(rank);
            unsafe {
                ptr::copy((*ptr).dims, dims.as_mut_ptr(), rank);
                dims.set_len(rank);
            };
            let nel: usize = dims.iter().product();
            let n = unsafe { ffi::Mat_VarGetNumberOfFields(ptr) } as usize;
            // fields name
            let field_names = unsafe {
                from_raw_parts(ffi::Mat_VarGetStructFieldnames(ptr), n)
                    .into_iter()
                    .map(|&s| CStr::from_ptr(s).to_str())
                    .collect::<std::result::Result<Vec<&str>, std::str::Utf8Error>>()?
            };
            // fields data pointer
            let field_ptr =
                unsafe { from_raw_parts((*ptr).data as *mut *mut ffi::matvar_t, n * nel) };
            let mut fields: Vec<Mat> = Vec::new();
            for (name, &ptr) in field_names.into_iter().cycle().zip(field_ptr.iter()) {
                let mat = Mat::from_ptr(name, ptr)?;
                fields.push(mat);
            }
            Ok(Mat {
                name: name.into(),
                matvar_t: ptr,
                fields: Some(fields),
                marker: PhantomData,
            })
        } else {
            Ok(Mat {
                name: name.into(),
                matvar_t: ptr,
                fields: None,
                marker: PhantomData,
            })
        }
    }
    /// Returns the field `name` from a Matlab structure
    pub fn field<S: Into<String>>(&self, name: S) -> Result<Vec<&Mat<'_>>> {
        let fields = if let Some(MatType::STRUCT) = self.mat_type() {
            self.fields.as_ref().unwrap()
        } else {
            return Err(MatioError::TypeMismatch(
                self.name.clone(),
                stringify!(MatType::STRUCT).to_string(),
                stringify!(self.mat_type()).to_string(),
            ));
        };
        let field_name: String = name.into();
        let field_value: Vec<&Mat> = fields
            .iter()
            .filter(|field| field.name == field_name)
            .collect();
        if field_value.is_empty() {
            Err(MatioError::FieldNotFound(field_name))
        } else {
            Ok(field_value)
        }
    }
}
