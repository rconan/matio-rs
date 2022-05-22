use crate::{Builder, MatVar, MatioError, Result, MatObjects};
use std::{marker::PhantomData, path::Path};

/// Mat file
pub struct MatFile {
    pub(crate) mat_t: *mut ffi::mat_t,
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
    /// Reads a variable `name` from the mat file
    fn read<T: 'static, S: Into<String>>(&self, name: S) -> Result<MatVar<T>>;
}
impl Load for MatFile {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        Builder::new(path).load()
    }
    fn read<T: 'static, S: Into<String>>(&self, name: S) -> Result<MatVar<T>> {
        let c_name = std::ffi::CString::new(name.into())?;
        let matvar_t = unsafe { ffi::Mat_VarRead(self.mat_t, c_name.as_ptr()) };
        if matvar_t.is_null() {
            Err(MatioError::MatVarRead(c_name.to_str().unwrap().to_string()))
        } else {
            MatVar {
                matvar_t,
                data_type: PhantomData,
            }
            .match_types()
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
    fn write(&self, mat_var: impl MatObjects) -> &Self;
}
impl Save for MatFile {
    fn save<P: AsRef<Path>>(path: P) -> Result<Self> {
        Builder::new(path).save()
    }
    fn write(&self, mut var: impl MatObjects) -> &Self {
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
