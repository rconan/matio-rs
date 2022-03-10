/*!
# Rust bindings and wrappers for [MATIO](https://github.com/tbeu/matio)

This crate provides bindings and wrappers for [MATIO](https://github.com/tbeu/matio):
MATLAB MAT file I/O C library

## Examples
Loading a mat file
```
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
# Ok::<(), matio_rs::MatioError>(())
```
Reading a scalar Matlab variable: a = π
```
# use matio_rs::{MatFile, Load};
# let mat_file = MatFile::load("data.mat")?;
if let Ok(mat) = mat_file.read("a") {
    println!("{mat}");
    let a: f64 = mat.into();
    println!("{a:?}");
}
# Ok::<(), matio_rs::MatioError>(())
```
Reading a Matlab vector: b = [3.0, 1.0, 4.0, 1.0, 6.0]
```
# use matio_rs::{MatFile, Load};
# let mat_file = MatFile::load("data.mat")?;
if let Ok(mat) = mat_file.read("b") {
    println!("{mat}");
    let b: Vec<f64> = mat.into();
    println!("{b:?}");
}
# Ok::<(), matio_rs::MatioError>(())
```
Reading a Matlab array: c = [4, 2; 3, 7]
```
# use matio_rs::{MatFile, Load};
# let mat_file = MatFile::load("data.mat")?;
if let Ok(mat) = mat_file.read("c") {
    println!("{mat}");
    let c: Vec<f64> = mat.into();
    println!("{c:?}");
}
# Ok::<(), matio_rs::MatioError>(())
```
Saving to a mat file
```
use matio_rs::{MatFile, MatVar, Save};
let mat_file = MatFile::save("data.rs.mat")?;
let b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
mat_file.write(MatVar::<f64>::new("a", 2f64.sqrt())?)
        .write(MatVar::<Vec<f64>>::new("b", b)?);
# Ok::<(), matio_rs::MatioError>(())
```
*/

use std::{
    any::{type_name, TypeId},
    fmt::Display,
    fs, io,
    marker::PhantomData,
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
    #[error("creating mat var {0} failed")]
    MatVarCreate(String),
    #[error("Rust ({0}) and Matlab ({1}) types do not match")]
    MatType(String, String),
}
pub type Result<T> = std::result::Result<T, MatioError>;

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

/// Mat file
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
    fn write<T>(&self, mat_var: MatVar<T>) -> &Self;
}
impl Save for MatFile {
    fn save<P: AsRef<Path>>(path: P) -> Result<Self> {
        Builder::new(path).save()
    }
    fn write<T>(&self, mat_var: MatVar<T>) -> &Self {
        unsafe {
            ffi::Mat_VarWrite(
                self.mat_t,
                mat_var.matvar_t,
                ffi::matio_compression_MAT_COMPRESSION_NONE,
            );
        }
        self
    }
}

/// Matlab variable
pub struct MatVar<T> {
    matvar_t: *mut ffi::matvar_t,
    data_type: PhantomData<T>,
}
impl<T> Drop for MatVar<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::Mat_VarFree(self.matvar_t);
        }
    }
}
impl<T> Display for MatVar<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { ffi::Mat_VarPrint(self.matvar_t, 0) }
        Ok(())
    }
}
impl<T: 'static> MatVar<T> {
    /// Checks Rust and Matlab types compatibility
    pub fn match_types(self) -> Result<Self> {
        unsafe {
            if (TypeId::of::<T>() == TypeId::of::<f64>()
                || TypeId::of::<T>() == TypeId::of::<Vec<f64>>())
                && (*self.matvar_t).data_type == ffi::matio_types_MAT_T_DOUBLE
            {
                return Ok(self);
            }
            Err(MatioError::MatType(
                type_name::<T>().to_string(),
                {
                    match (*self.matvar_t).data_type {
                        ffi::matio_types_MAT_T_DOUBLE => "DOUBLE",
                        _ => "UNKNOWN",
                    }
                }
                .to_string(),
            ))
        }
    }
}

impl MatVar<f64> {
    /// Creates a new Matlab variable `name`
    pub fn new<S: Into<String>>(name: S, mut data: f64) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut dims = [1, 1];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                ffi::matio_classes_MAT_C_DOUBLE,
                ffi::matio_types_MAT_T_DOUBLE,
                2,
                dims.as_mut_ptr(),
                &mut data as *mut _ as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatVar {
                matvar_t,
                data_type: PhantomData,
            })
        }
    }
}
impl MatVar<Vec<f64>> {
    /// Creates a new Matlab variable `name`
    pub fn new<S: Into<String>>(name: S, data: &mut [f64]) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut dims = [1, data.len() as u64];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                ffi::matio_classes_MAT_C_DOUBLE,
                ffi::matio_types_MAT_T_DOUBLE,
                2,
                dims.as_mut_ptr(),
                data.as_mut_ptr() as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatVar {
                matvar_t,
                data_type: PhantomData,
            })
        }
    }
    pub fn array<S: Into<String>>(
        name: S,
        data: &mut [f64],
        shape: (usize, usize),
    ) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut dims = [shape.0 as u64, shape.1 as u64];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                ffi::matio_classes_MAT_C_DOUBLE,
                ffi::matio_types_MAT_T_DOUBLE,
                2,
                dims.as_mut_ptr(),
                data.as_mut_ptr() as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatVar {
                matvar_t,
                data_type: PhantomData,
            })
        }
    }
}

impl From<MatVar<f64>> for f64 {
    fn from(mat_var: MatVar<f64>) -> Self {
        unsafe { ((*mat_var.matvar_t).data as *mut f64).read() }
    }
}

impl From<MatVar<Vec<f64>>> for Vec<f64> {
    fn from(mat_var: MatVar<Vec<f64>>) -> Self {
        unsafe {
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
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let _mat_file = MatFile::load("data.mat").unwrap();
    }

    #[test]
    fn test_read_scalar() {
        let mat_file = MatFile::load("data.mat").unwrap();
        let mat = mat_file.read("a").unwrap();
        let a: f64 = mat.into();
        assert_eq!(a, std::f64::consts::PI);
    }

    #[test]
    fn test_read_1d() {
        let mat_file = MatFile::load("data.mat").unwrap();
        let mat = mat_file.read("b").unwrap();
        let b: Vec<f64> = mat.into();
        assert_eq!(b, vec![3f64, 1., 4., 1., 6.])
    }

    #[test]
    fn test_read_2d() {
        let mat_file = MatFile::load("data.mat").unwrap();
        let mat = mat_file.read("c").unwrap();
        let c: Vec<f64> = mat.into();
        assert_eq!(c, vec![4f64, 3., 2., 7.])
    }

    #[test]
    fn test_save() {
        let b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
        {
            let mat_file = MatFile::save("data.rs.mat").unwrap();
            mat_file.write(MatVar::<f64>::new("a", 2f64.sqrt()).unwrap());
            mat_file.write(MatVar::<Vec<f64>>::new("b", b.clone()).unwrap());
        }
        let mat_file = MatFile::load("data.rs.mat").unwrap();
        let mat = mat_file.read("a").unwrap();
        let a: f64 = mat.into();
        assert_eq!(a, 2f64.sqrt());
        let mat = mat_file.read("b").unwrap();
        let bb: Vec<f64> = mat.into();
        assert_eq!(b, bb);
    }
}
