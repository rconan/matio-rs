use crate::{
    MatFile, MatFileRead, MatFileWrite, MatTryFrom, MatTryInto, MatType, MatioError, Result,
};
use std::{ffi::CStr, marker::PhantomData, ptr, slice::from_raw_parts};

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
    pub fn read<S: Into<String>>(&self, name: S) -> Result<Mat<'a>> {
        let c_name = std::ffi::CString::new(name.into())?;
        let matvar_t = unsafe { ffi::Mat_VarRead(self.mat_t, c_name.as_ptr()) };
        if matvar_t.is_null() {
            Err(MatioError::MatVarRead(c_name.to_str().unwrap().to_string()))
        } else {
            Mat::from_ptr(c_name.to_str()?, matvar_t)
        }
    }
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
    pub fn var<S: Into<String>, T>(&self, name: S) -> Result<T>
    where
        Mat<'a>: MatTryInto<T>,
    {
        self.read(name).and_then(|mat| mat.maybe_into())
    }
}
impl<'a> MatFileWrite<'a> {
    pub fn var<S: Into<String>, T>(&self, name: S, data: T) -> Result<&Self>
    where
        Mat<'a>: MatTryFrom<'a, T>,
    {
        let mat: Mat<'a> = MatTryFrom::<'a, T>::maybe_from(name, data)?;
        self.write(mat);
        Ok(self)
    }
}
impl<'a> Mat<'a> {
    pub fn rank(&self) -> usize {
        unsafe { (*self.matvar_t).rank as usize }
    }
    pub fn dims(&self) -> Vec<u64> {
        let rank = self.rank();
        let mut dims: Vec<u64> = Vec::with_capacity(rank);
        unsafe {
            ptr::copy((*self.matvar_t).dims, dims.as_mut_ptr(), rank);
            dims.set_len(rank);
        };
        dims
    }
    pub fn len(&self) -> usize {
        self.dims().into_iter().product::<u64>() as usize
    }
    pub fn mat_type(&self) -> MatType {
        MatType::from_ptr(self.matvar_t)
    }
    pub fn from_ptr<S: Into<String>>(name: S, ptr: *mut ffi::matvar_t) -> Result<Self> {
        if MatType::from_ptr(ptr) != MatType::STRUCT {
            return Ok(Mat {
                name: name.into(),
                matvar_t: ptr,
                fields: None,
                marker: PhantomData,
            });
        }

        let rank = unsafe { (*ptr).rank as usize };
        let mut dims: Vec<u64> = Vec::with_capacity(rank);
        unsafe {
            ptr::copy((*ptr).dims, dims.as_mut_ptr(), rank);
            dims.set_len(rank);
        };
        let nel: u64 = dims.iter().product();
        let n = unsafe { ffi::Mat_VarGetNumberOfFields(ptr) } as usize;
        // fields name
        let field_names = unsafe {
            from_raw_parts(ffi::Mat_VarGetStructFieldnames(ptr) as *mut *mut i8, n)
                .into_iter()
                .map(|&s| CStr::from_ptr(s).to_str())
                .collect::<std::result::Result<Vec<&str>, std::str::Utf8Error>>()?
        };
        // fields data pointer
        let field_ptr =
            unsafe { from_raw_parts((*ptr).data as *mut *mut ffi::matvar_t, n * nel as usize) };
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
    }
    pub fn field<S: Into<String>>(&'a self, name: S) -> Result<Vec<&'a Mat>> {
        let fields = if self.mat_type() == MatType::STRUCT {
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
