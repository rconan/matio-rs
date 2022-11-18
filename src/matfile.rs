use crate::{MatioError, Result};
use std::{fs, io, marker::PhantomData, ops::Deref, path::Path, ptr};

/// Mat file
pub struct MatFile<'a> {
    pub(crate) mat_t: *mut ffi::mat_t,
    marker: PhantomData<&'a ffi::mat_t>,
}
/// Mat file [Mat](crate::Mat) reader
pub struct MatFileRead<'a>(MatFile<'a>);
impl<'a> Deref for MatFileRead<'a> {
    type Target = MatFile<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// Mat file [Mat](crate::Mat) writer
pub struct MatFileWrite<'a>(MatFile<'a>);
impl<'a> Deref for MatFileWrite<'a> {
    type Target = MatFile<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> MatFile<'a> {
    pub(crate) fn from_ptr(mat_t: *mut ffi::mat_t) -> MatFile<'a> {
        MatFile {
            mat_t,
            marker: PhantomData,
        }
    }
    /// Loads [Mat](crate::Mat) variables from a mat file
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
    /// Saves [Mat](crate::Mat) variables to a mat file
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
