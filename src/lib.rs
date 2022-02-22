use ffi;
use std::{
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
    ptr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatioError {
    #[error("mat file does not exists")]
    NoFile(#[from] io::Error),
    #[error("opening mat file {0} failed")]
    MatOpen(String),
    #[error("mat file name can't be processed")]
    MatName(#[from] std::ffi::NulError),
    #[error("reading mat var {0} failed")]
    MatVarRead(String),
}
pub type Result<T> = std::result::Result<T, MatioError>;

pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

pub struct Loader {
    mat_name: PathBuf,
    access_mode: AccessMode,
}
impl Loader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            mat_name: path.as_ref().to_path_buf(),
            access_mode: AccessMode::ReadOnly,
        }
    }
    pub fn read_only(self) -> Self {
        Self {
            access_mode: AccessMode::ReadOnly,
            ..self
        }
    }
    pub fn read_write(self) -> Self {
        Self {
            access_mode: AccessMode::ReadWrite,
            ..self
        }
    }
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
}

pub struct MatFile {
    mat_t: *mut ffi::mat_t,
}
impl Drop for MatFile {
    fn drop(&mut self) {
        if unsafe { ffi::Mat_Close(self.mat_t) } != 0 {
            panic!("MatFile::Drop failed")
        }
    }
}
impl MatFile {
    pub fn read<S: Into<String>>(&self, name: S) -> Result<MatVar> {
        let c_name = std::ffi::CString::new(name.into())?;
        let matvar_t = unsafe { ffi::Mat_VarRead(self.mat_t, c_name.as_ptr()) };
        if matvar_t.is_null() {
            Err(MatioError::MatVarRead(c_name.to_str().unwrap().to_string()))
        } else {
            Ok(MatVar { matvar_t })
        }
    }
}

pub struct MatVar {
    matvar_t: *mut ffi::matvar_t,
}
impl Drop for MatVar {
    fn drop(&mut self) {
        unsafe {
            ffi::Mat_VarFree(self.matvar_t);
        }
    }
}
impl Display for MatVar {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { ffi::Mat_VarPrint(self.matvar_t, 0) }
        Ok(())
    }
}

impl From<MatVar> for Option<f64> {
    fn from(mat_var: MatVar) -> Self {
        unsafe {
            if (*mat_var.matvar_t).data_type == 9 {
                Some(((*mat_var.matvar_t).data as *mut f64).read())
            } else {
                println!("The Matlab var type do not match the expect Rust type");
                None
            }
        }
    }
}

impl From<MatVar> for Option<Vec<f64>> {
    fn from(mat_var: MatVar) -> Self {
        unsafe {
            if (*mat_var.matvar_t).data_type == 9 {
                let rank = (*mat_var.matvar_t).rank as usize;
                let mut dims: Vec<u64> = Vec::with_capacity(rank);
                ptr::copy((*mat_var.matvar_t).dims, dims.as_mut_ptr(), rank);
                dims.set_len(rank);
                let length = dims.into_iter().product::<u64>() as usize;

                let mut value: Vec<f64> = Vec::with_capacity(length);
                ptr::copy(
                    (*mat_var.matvar_t).data as *mut f64,
                    value.as_mut_ptr(),
                    length,
                );
                value.set_len(length);
                Some(value)
            } else {
                println!("The Matlab var type do not match the expect Rust type");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader() {
        let _mat_file = Loader::new("data.mat").load().unwrap();
    }

    #[test]
    fn test_read_scalar() {
        let mat_file = Loader::new("data.mat").load().unwrap();
        if let Ok(mat) = mat_file.read("a") {
            println!("{mat}");
            let a: Option<f64> = mat.into();
            println!("{a:?}");
        }
    }

    #[test]
    fn test_read_1d() {
        let mat_file = Loader::new("data.mat").load().unwrap();
        if let Ok(mat) = mat_file.read("b") {
            println!("{mat}");
            let b: Option<Vec<f64>> = mat.into();
            println!("{b:?}");
        }
    }

    #[test]
    fn test_read_2d() {
        let mat_file = Loader::new("data.mat").load().unwrap();
        if let Ok(mat) = mat_file.read("c") {
            println!("{mat}");
            let c: Option<Vec<f64>> = mat.into();
            println!("{c:?}");
        }
    }
}
