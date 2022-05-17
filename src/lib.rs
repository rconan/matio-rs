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
*/

use std::io;
use thiserror::Error;

mod builder;
pub use builder::Builder;
mod matfile;
pub use matfile::{Load, MatFile, Save};
mod matvar;
pub use matvar::MatVar;

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
