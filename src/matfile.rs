use crate::{matvar::DataType, Builder, MatObject, MatStruct, MatVar, MatioError, Result};
use std::{marker::PhantomData, path::Path};

/// Mat file
pub struct MatFile {
    pub(crate) mat_t: *mut ffi::mat_t,
}
impl MatFile {
    pub fn read_ptr<S: Into<String>>(&self, name: S) -> Result<*mut ffi::matvar_t> {
        let c_name = std::ffi::CString::new(name.into())?;
        unsafe {
            let matvar_t = ffi::Mat_VarRead(self.mat_t, c_name.as_ptr());
            if matvar_t.is_null() {
                Err(MatioError::MatVarRead(c_name.to_str().unwrap().to_string()))
            } else {
                Ok(matvar_t)
            }
        }
    }
}
impl Drop for MatFile {
    fn drop(&mut self) {
        if unsafe { ffi::Mat_Close(self.mat_t) } != 0 {
            panic!("MatFile::Drop failed")
        }
    }
}
/// Mat file loading interface
pub trait Load {
    /// Loads a mat file from `path`
    fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized;
}
impl Load for MatFile {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        Builder::new(path).load()
    }
}
/// Matlab variable reading interface
pub trait Read<M> {
    /// Reads a variable `name` from the mat file
    fn read<S: Into<String> + Clone>(&self, name: S) -> Result<M>;
}
impl<T: 'static + DataType + Copy> Read<MatVar<T>> for MatFile {
    fn read<S: Into<String> + Clone>(&self, name: S) -> Result<MatVar<T>> {
        unsafe {
            let matvar_t = self.read_ptr(name.clone())?;
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
    fn read<S: Into<String> + Clone>(&self, name: S) -> Result<MatVar<Vec<T>>> {
        unsafe {
            let matvar_t = self.read_ptr(name.clone())?;
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
    fn read<S: Into<String> + Clone>(&self, name: S) -> Result<MatStruct> {
        unsafe {
            let matstruct_t = self.read_ptr(name.clone())?;
            if (*matstruct_t).class_type == MatStruct::mat_c()
                && (*matstruct_t).data_type == MatStruct::mat_t()
            {
                Ok(MatStruct {
                    matvar_t: matstruct_t,
                    fields: None,
                })
            } else {
                Err(MatioError::MatVarRead(name.into()))
            }
        }
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
impl Save for MatFile {
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
