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
}

/// Matlab file high-level interface to [Load]
pub trait Get<T> {
    /// Gets the variable `name` from a [MatFile] into a Rust data type
    fn get<S: Into<String> + Clone>(&self, name: S) -> Result<T>;
}
impl<T> Get<T> for MatFile
where
    MatFile: Read<MatVar<T>>,
    MatVar<T>: Into<T>,
{
    fn get<S: Into<String> + Clone>(&self, name: S) -> Result<T> {
        self.read(name).map(|mat_var| mat_var.into())
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

/// Matlab file high-level interface to [Save]
pub trait Set<T> {
    /// Sets a Rust variable into a [MatFile] with `name`
    fn set<S>(&self, name: S, data: &T) -> &Self
    where
        (S, T): Into<MatVar<T>>,
        S: Into<String>;
}
impl<T> Set<T> for MatFile
where
    T: Clone,
    MatFile: Save,
{
    fn set<S>(&self, name: S, data: &T) -> &Self
    where
        (S, T): Into<MatVar<T>>,
        S: Into<String>,
    {
        self.write((name, data.clone()).into())
    }
}
