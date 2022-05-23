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
let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
mat_file.write(MatVar::<f64>::new("a", 2f64.sqrt())?)
        .write(MatVar::<Vec<f64>>::new("b", &mut b)?);
# Ok::<(), matio_rs::MatioError>(())
```
Writing a Matlab structure to a mat file
```
use matio_rs::{MatFile, MatStruct, Save, Field};
let mat = MatStruct::new("s")
            .field("fa", &123f64)?
            .field("fb", &vec![0i32, 1, 2, 3, 4])?
            .build()?;
let mat_file = MatFile::save("struct.mat")?;
mat_file.write(mat);
# Ok::<(), matio_rs::MatioError>(())
```
Writing a Matlab structure array to a mat file
```
use matio_rs::{MatFile, MatStruct, Save, FieldIterator};
let u = vec![1u32,2,3];
let v: Vec<_> = u.iter()
                  .map(|&x| (0..x).map(|y| y as f64 *(x as f64)/5.).collect::<Vec<f64>>())
                  .collect();
let mat = MatStruct::new("s")
            .field("fa", u.iter())?
            .field("fb", v.iter())?
            .build()?;
let mat_file = MatFile::save("struct-array.mat")?;
mat_file.write(mat);
# Ok::<(), matio_rs::MatioError>(())
```*/

use std::io;
use thiserror::Error;

mod builder;
pub use builder::Builder;
mod matfile;
pub use matfile::{Load, MatFile, Save};
mod matvar;
pub use matvar::MatVar;
mod matstruct;
pub use matstruct::{Field, FieldIterator, MatStruct};

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
    #[error("structure fields missing")]
    NoFields,
    #[error("structure fields have different sizes {0:?}")]
    FieldSize(Vec<usize>),
}
pub type Result<T> = std::result::Result<T, MatioError>;

/// Interface to Matlab data
pub trait MatObject {
    fn as_mut_ptr(&mut self) -> *mut ffi::matvar_t;
}

#[cfg(test)]
mod tests {
    use crate::{Load, MatFile, MatStruct, MatVar, Save};

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
        let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
        {
            let mat_file = MatFile::save("data.rs.mat").unwrap();
            mat_file.write(MatVar::<f64>::new("a", 2f64.sqrt()).unwrap());
            mat_file.write(MatVar::<Vec<f64>>::new("b", &mut b).unwrap());
        }
        let mat_file = MatFile::load("data.rs.mat").unwrap();
        let mat = mat_file.read("a").unwrap();
        let a: f64 = mat.into();
        assert_eq!(a, 2f64.sqrt());
        let mat = mat_file.read("b").unwrap();
        let bb: Vec<f64> = mat.into();
        assert_eq!(b, bb);
    }

    #[test]
    fn test_save_polytype() {
        let mat_file = MatFile::save("data-poly.mat").unwrap();
        mat_file.write(MatVar::<i8>::new("a", 1i8).unwrap());
        mat_file.write(MatVar::<f32>::new("b", 2f32).unwrap());
        mat_file.write(MatVar::<Vec<u16>>::new("c", &mut [3u16; 3]).unwrap());
    }

    #[test]
    fn test_save_struct() {
        use crate::Field;
        let mat = MatStruct::new("a")
            .field("fa", &10f64)
            .unwrap()
            .field("fb", &vec![0i32, 1, 2, 3])
            .unwrap()
            .build()
            .unwrap();
        let mat_file = MatFile::save("struct.mat").unwrap();
        mat_file.write(mat);
    }

    #[test]
    fn test_save_struct_array() {
        use crate::FieldIterator;
        let u = vec![1, 2, 3];
        let v = vec![4, 5, 6];
        let mat = MatStruct::new("a")
            .field("fa", u.iter())
            .unwrap()
            .field("fb", v.iter())
            .unwrap()
            .build()
            .unwrap();
        let mat_file = MatFile::save("struct.mat").unwrap();
        mat_file.write(mat);
    }
    #[cfg(feature = "nalgebra")]
    #[test]
    fn test_vector() {
        let mat_file = MatFile::load("arrays.mat").unwrap();
        let mat = mat_file.read("a").unwrap();
        let a: nalgebra::DVector<f64> = mat.into();
        println!("{a}");
        let mat = mat_file.read("b").unwrap();
        let b: nalgebra::DVector<f64> = mat.into();
        println!("{b}");
    }

    #[cfg(feature = "nalgebra")]
    #[test]
    fn test_matrix() {
        let mat_file = MatFile::load("arrays.mat").unwrap();
        let mat = mat_file.read("a").unwrap();
        let a: Option<nalgebra::DMatrix<f64>> = mat.into();
        println!("{:}", a.unwrap());
        let mat = mat_file.read("b").unwrap();
        let b: Option<nalgebra::DMatrix<f64>> = mat.into();
        println!("{b:?}");
    }
}
