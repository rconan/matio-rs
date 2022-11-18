use crate::{MatioError, Result};
use std::{fs, io, marker::PhantomData, ops::Deref, path::Path, ptr};

/// Mat file
pub struct MatFile<'a> {
    pub(crate) mat_t: *mut ffi::mat_t,
    marker: PhantomData<&'a ffi::mat_t>,
}
pub struct MatFileRead<'a>(MatFile<'a>);
impl<'a> Deref for MatFileRead<'a> {
    type Target = MatFile<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct MatFileWrite<'a>(MatFile<'a>);
impl<'a> Deref for MatFileWrite<'a> {
    type Target = MatFile<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> MatFile<'a> {
    pub fn from_ptr(mat_t: *mut ffi::mat_t) -> MatFile<'a> {
        MatFile {
            mat_t,
            marker: PhantomData,
        }
    }
    /// Loads a mat file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<MatFileRead<'a>> {
        let attrs = fs::metadata(&path)?;
        if attrs.is_file() {
            let mat_name = std::ffi::CString::new(path.as_ref().to_str().unwrap())?;
            let mat_t =
                unsafe { ffi::Mat_Open(mat_name.as_ptr(), ffi::mat_acc_MAT_ACC_RDONLY as i32) };
            if mat_t.is_null() {
                Err(MatioError::MatOpen(
                    path.as_ref().to_str().unwrap().to_string(),
                ))
            } else {
                Ok(MatFileRead(MatFile::from_ptr(mat_t)))
            }
        } else {
            Err(MatioError::NoFile(io::Error::new(
                io::ErrorKind::NotFound,
                format!("mat file {:?} not found", path.as_ref()),
            )))
        }
    }
    pub fn save<P: AsRef<Path>>(path: P) -> Result<MatFileWrite<'a>> {
        let mat_name = std::ffi::CString::new(path.as_ref().to_str().unwrap())?;
        let mat_t =
            unsafe { ffi::Mat_CreateVer(mat_name.as_ptr(), ptr::null(), ffi::mat_ft_MAT_FT_MAT5) };
        if mat_t.is_null() {
            Err(MatioError::MatOpen(
                path.as_ref().to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatFileWrite(MatFile::from_ptr(mat_t)))
        }
    }
}

impl<'a> Drop for MatFile<'a> {
    fn drop(&mut self) {
        if unsafe { ffi::Mat_Close(self.mat_t) } != 0 {
            panic!("failed to close matfile")
        }
    }
}
/* /// Mat file loading interface
pub trait Load {
    /// Loads a mat file from `path`
    fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized;
}
impl<'a> Load for MatFile<'a, Open> {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        Builder::new(path).load()
    }
}

/// Mat file saving interface
pub trait Save {
    /// saves a mat file to `path`
    fn save<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized;
    /// Writes a Matlab variable to the mat file
    fn write(&self, mat_var: impl MatObject) -> &Self;
}
impl<'a> Save for MatFile<'a, Open> {
    fn save<P: AsRef<Path>>(path: P) -> Result<Self> {
        Builder::new(path).save()
    }
    fn write(&self, mut var: impl MatObject) -> &Self {
        unsafe {
            ffi::Mat_VarWrite(
                self.mat_t,
                var.as_mut_ptr(),
                ffi::matio_compression_MAT_COMPRESSION_NONE,
            );
        }
        self
    }
}
 */
/* /// Matlab variable reading interface
pub trait Read<M> {
    /// Reads a variable `name` from the mat file
    fn read<S: Into<String> + Clone>(&self, name: S) -> Result<M>;
}
impl<T: 'static + DataType + Copy> Read<MatVar<T>> for MatFile {
    fn read<S: Into<String>>(&self, name: S) -> Result<MatVar<T>> {
        let name: String = name.into();
        unsafe {
            let matvar_t = self.read_ptr(name.as_str())?;
            if (*matvar_t).class_type == T::mat_c() && (*matvar_t).data_type == T::mat_t() {
                Ok(MatVar {
                    matvar_t,
                    data_type: PhantomData,
                })
            } else {
                Err(MatioError::MatVarRead(name.into()))
            }
        }
    }
}
impl<T: 'static + DataType> Read<MatVar<Vec<T>>> for MatFile {
    fn read<S: Into<String>>(&self, name: S) -> Result<MatVar<Vec<T>>> {
        let name: String = name.into();
        unsafe {
            let matvar_t = self.read_ptr(name.as_str())?;
            if (*matvar_t).class_type == T::mat_c() && (*matvar_t).data_type == T::mat_t() {
                Ok(MatVar {
                    matvar_t,
                    data_type: PhantomData,
                })
            } else {
                Err(MatioError::MatVarRead(name.into()))
            }
        }
    }
}
impl Read<MatStruct> for MatFile {
    fn read<S: Into<String>>(&self, name: S) -> Result<MatStruct> {
        let name: String = name.into();
        unsafe {
            let matstruct_t = self.read_ptr(name.as_str())?;
            if (*matstruct_t).class_type == MatStruct::mat_c()
                && (*matstruct_t).data_type == MatStruct::mat_t()
            {
                Ok(MatStruct {
                    matvar_t: matstruct_t,
                    fields: None,
                })
            } else {
                Err(MatioError::MatVarRead(name))
            }
        }
    }
} */

/* /// Matlab file high-level interface to [Load]
pub trait Get<T> {
    /// Gets the variable `name` from a [MatFile] into a Rust data type
    fn var<S: Into<String> + Clone>(&self, name: S) -> Result<T>;
}
impl<T> Get<T> for MatFile
where
    MatFile: Read<MatVar<T>>,
    MatVar<T>: Into<T>,
{
    fn var<S: Into<String> + Clone>(&self, name: S) -> Result<T> {
        self.read(name).map(|mat_var| mat_var.into())
    }
} */

/* /// Matlab file high-level interface to [Save]
pub trait Set<'a, T>
where
    T: 'a,
{
    /// Sets a Rust variable into a [MatFile] with `name`
    fn var<S>(&'a self, name: S, data: &'a T) -> &'a Self
    where
        (S, &'a T): Into<MatVar<T>>,
        S: Into<String>;
}
impl<'a, T> Set<'a, T> for MatFile
where
    T: 'a,
    MatFile: Save,
{
    fn var<S>(&'a self, name: S, data: &'a T) -> &'a Self
    where
        (S, &'a T): Into<MatVar<T>>,
        S: Into<String>,
    {
        self.write((name, data).into())
    }
}
pub trait SetStruct<'a, T>
where
    T: 'a,
{
    /// Sets a Rust struct into a [MatFile] with `name`
    fn mat_struct<S>(&'a self, name: S, data: &'a T) -> &'a Self
    where
        &'a T: Into<MatStruct>,
        S: Into<String>;
}
 */
