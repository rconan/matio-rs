use crate::{MatFile, MatioError, Result};
use std::{
    fs, io,
    path::{Path, PathBuf},
    ptr,
};

/// Mat file acces modes
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

/// Mat file builder
pub struct Builder {
    mat_name: PathBuf,
    access_mode: AccessMode,
}
impl Builder {
    /// Creates a new mat file loader object from the `path`
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            mat_name: path.as_ref().to_path_buf(),
            access_mode: AccessMode::ReadOnly,
        }
    }
    /// Sets the access mode to read-only (default)
    pub fn read_only(self) -> Self {
        Self {
            access_mode: AccessMode::ReadOnly,
            ..self
        }
    }
    /// Sets the access mode to read-write
    pub fn read_write(self) -> Self {
        Self {
            access_mode: AccessMode::ReadWrite,
            ..self
        }
    }
    /// Loads a mat file
    pub fn load(self) -> Result<MatFile> {
        let attrs = fs::metadata(&self.mat_name)?;
        if attrs.is_file() {
            let mat_name = std::ffi::CString::new(self.mat_name.to_str().unwrap())?;
            let mat_t = unsafe { ffi::Mat_Open(mat_name.as_ptr(), self.access_mode as i32) };
            if mat_t.is_null() {
                Err(MatioError::MatOpen(
                    self.mat_name.to_str().unwrap().to_string(),
                ))
            } else {
                Ok(MatFile { mat_t })
            }
        } else {
            Err(MatioError::NoFile(io::Error::new(
                io::ErrorKind::NotFound,
                format!("mat file {} not found", self.mat_name.to_str().unwrap()),
            )))
        }
    }
    pub fn save(self) -> Result<MatFile> {
        let mat_name = std::ffi::CString::new(self.mat_name.to_str().unwrap())?;
        let mat_t =
            unsafe { ffi::Mat_CreateVer(mat_name.as_ptr(), ptr::null(), ffi::mat_ft_MAT_FT_MAT5) };
        if mat_t.is_null() {
            Err(MatioError::MatOpen(
                self.mat_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatFile { mat_t })
        }
    }
}
