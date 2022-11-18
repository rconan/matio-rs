/*!
# Rust bindings and wrappers for [MATIO](https://github.com/tbeu/matio)

This crate provides bindings and wrappers for [MATIO](https://github.com/tbeu/matio):
MATLAB MAT file I/O C library

## Examples
Loading a mat file
```
use matio_rs::{MatFile, Load};
use std::path::Path;
let data_path = Path::new("data").join("data").with_extension("mat");
let mat_file = MatFile::load(data_path)?;
# Ok::<(), matio_rs::MatioError>(())
```
Reading a scalar Matlab variable: a = π
```
use matio_rs::{MatFile, Load, Get};
use std::path::Path;
let data_path = Path::new("data").join("data").with_extension("mat");
let a: f64 = MatFile::load(data_path)?.var("a")?;
println!("{a:?}");
# Ok::<(), matio_rs::MatioError>(())
```
Reading a Matlab vector: b = [3.0, 1.0, 4.0, 1.0, 6.0]
```
use matio_rs::{MatFile, Load, Get};
use std::path::Path;
let data_path = Path::new("data").join("data").with_extension("mat");
let b: Vec<f64> = MatFile::load(data_path)?.var("b")?;
println!("{b:?}");
# Ok::<(), matio_rs::MatioError>(())
```
Reading a Matlab array: c = [4, 2; 3, 7]
```
use matio_rs::{MatFile, Load, Get};
use std::path::Path;
let data_path = Path::new("data").join("data").with_extension("mat");
let c: Vec<f64> = MatFile::load(data_path)?.var("c")?;
println!("{c:?}");
# Ok::<(), matio_rs::MatioError>(())
```
Saving to a mat file
```
use matio_rs::{MatFile, Save, Set};
use std::path::Path;
let data_path = Path::new("data").join("data-rs").with_extension("mat");
let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
MatFile::save(data_path)?
    .var("a", &2f64.sqrt())
    .var("b", &b);
# Ok::<(), matio_rs::MatioError>(())
```
Writing a Matlab structure to a mat file
```
use matio_rs::{MatFile, MatStruct, Save, Field};
use std::path::Path;
let mat = MatStruct::new("s")
            .field("fa", &123f64)?
            .field("fb", &vec![0i32, 1, 2, 3, 4])?
            .build()?;
let data_path = Path::new("data").join("struct").with_extension("mat");
let mat_file = MatFile::save(data_path)?;
mat_file.write(mat);
# Ok::<(), matio_rs::MatioError>(())
```
Writing a Matlab structure array to a mat file
```
use matio_rs::{MatFile, MatStruct, Save, FieldIterator};
use std::path::Path;
let u = vec![1u32,2,3];
let v: Vec<_> = u.iter()
                  .map(|&x| (0..x).map(|y| y as f64 *(x as f64)/5.).collect::<Vec<f64>>())
                  .collect();
let mat = MatStruct::new("s")
            .field("fa", u.iter())?
            .field("fb", v.iter())?
            .build()?;
let data_path = Path::new("data").join("struct-array").with_extension("mat");
let mat_file = MatFile::save(data_path)?;
mat_file.write(mat);
# Ok::<(), matio_rs::MatioError>(())
```
Writing a nested Matlab structure to a mat file
```
use matio_rs::{MatFile, MatStruct, MatStructBuilder, Save};
use std::path::Path;
let mut builder = {
    use matio_rs::Field;
    MatStruct::new("a")
        .field("fa", &10f64)?
        .field("fb", &vec![0i32, 1, 2, 3])?
};
let nested = {
    use matio_rs::Field;
    MatStruct::new("a")
        .field("fa", &10f64)?
        .field("fb", &vec![0i32, 1, 2, 3])?
        .build()?
};
builder = <MatStructBuilder as matio_rs::FieldMatObject<MatStruct>>::field(
    builder, "nested", nested,
)?;
let data_path = Path::new("data").join("struct_nested").with_extension("mat");
let mat_file = MatFile::save(data_path).unwrap();
mat_file.write(builder.build()?);
# Ok::<(), matio_rs::MatioError>(())
```
Loading Matlab array into [nalgebra](https://docs.rs/nalgebra) vectors
```
use matio_rs::{MatFile, Load, Read, MatVar};
use std::path::Path;
let data_path = Path::new("data").join("arrays").with_extension("mat");
let mat_file = MatFile::load(data_path)?;
let a: nalgebra::DVector<f64> =
    <MatFile as Read<MatVar<Vec<f64>>>>::read(&mat_file,"a")?.into();
println!("{a}");
let b: nalgebra::DVector<f64> =
    <MatFile as Read<MatVar<Vec<f64>>>>::read(&mat_file,"b")?.into();
println!("{b}");
# Ok::<(), matio_rs::MatioError>(())
```
Loading Matlab array into [nalgebra](https://docs.rs/nalgebra) matrices
```
use matio_rs::{MatFile, Load, Read, MatVar};
use std::path::Path;
let data_path = Path::new("data").join("arrays").with_extension("mat");
let mat_file = MatFile::load(data_path)?;
let a: Option<nalgebra::DMatrix<f64>> =
    <MatFile as Read<MatVar<Vec<f64>>>>::read(&mat_file,"a")?.into();
println!("{a:?}");
let b: Option<nalgebra::DMatrix<f64>> =
    <MatFile as Read<MatVar<Vec<f64>>>>::read(&mat_file,"b")?.into();
println!("{b:?}");
# Ok::<(), matio_rs::MatioError>(())
```*/

use std::io;
use thiserror::Error;

// mod builder;
// pub use builder::Builder;
mod matfile;
pub use matfile::{MatFile, MatFileRead, MatFileWrite};
mod datatype;
use datatype::{DataType, MatType};
mod mat;
pub use mat::Mat;
mod convert;
pub use convert::{MatTryFrom, MatTryInto};

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
    #[error("Matlab var. {0}: expected Matlab type {1} found {2}")]
    TypeMismatch(String, String, String),
    #[error("Matlab var. {0}: cannot convert a Matlab array of length {0} into a Rust scalar")]
    Scalar(String, usize),
    #[error("Field name cannot be converted to &str")]
    FieldName(#[from] std::str::Utf8Error),
    #[error("Field {0} not found")]
    FieldNotFound(String),
}
pub type Result<T> = std::result::Result<T, MatioError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    pub fn root() -> PathBuf {
        Path::new("data").into()
    }

    #[test]
    fn test_read_scalar() {
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let a: f64 = mat_file.var("a").unwrap();
        assert_eq!(a, std::f64::consts::PI);
    }

    #[test]
    fn test_read_1d() {
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let b: Vec<f64> = mat_file.var("b").unwrap();
        assert_eq!(b, vec![3f64, 1., 4., 1., 6.])
    }

    #[test]
    fn test_get_2d() {
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let c: Vec<f64> = mat_file.var("c").unwrap();
        assert_eq!(c, vec![4f64, 3., 2., 7.])
    }

    #[test]
    fn test_readwrite() {
        let b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
        MatFile::save(root().join("data.rs.mat"))
            .unwrap()
            .var("a", 2f64.sqrt())
            .unwrap()
            .var("b", &b)
            .unwrap();
        let mat_file = MatFile::load(root().join("data.rs.mat")).unwrap();
        let a: f64 = mat_file.var("a").unwrap();
        assert_eq!(a, 2f64.sqrt());
        let bb: Vec<f64> = mat_file.var("b").unwrap();
        assert_eq!(b, bb);
    }

    fn polytype() {
        MatFile::save(root().join("data-poly.mat"))
            .unwrap()
            .var("a", 1i8)
            .unwrap()
            .var("b", 2f32)
            .unwrap()
            .var("c", &vec![3u16; 3])
            .unwrap();
    }

    #[test]
    fn test_polytype() {
        polytype();
        let mat_file = MatFile::load(root().join("data-poly.mat")).unwrap();
        let a: i8 = mat_file.var("a").unwrap();
        assert_eq!(a, 1i8);
        let b: f32 = mat_file.var("b").unwrap();
        assert_eq!(b, 2f32);
        let c: Vec<u16> = mat_file.var("c").unwrap();
        assert_eq!(c, vec![3u16; 3]);
    }

    fn save_struct() {
        let mat_a = Mat::maybe_from("fa", 123f64).unwrap();
        let v = vec![0i32, 1, 2, 3, 4];
        let mat_v = Mat::maybe_from("fb", &v).unwrap();

        let data = vec![mat_a, mat_v];
        let mat_struct = Mat::maybe_from("s", data).unwrap();

        let mat_file = MatFile::save(root().join("struct.mat")).unwrap();
        mat_file.write(mat_struct);
    }

    #[test]
    fn test_struct() {
        save_struct();
        let mat_file = MatFile::load(root().join("struct.mat")).unwrap();
        let mat: Mat = mat_file.var("s").unwrap();
        let a: f64 = mat
            .field("fa")
            .unwrap()
            .get(0)
            .unwrap()
            .maybe_into()
            .unwrap();
        assert_eq!(a, 123f64);
        let b: Vec<i32> = mat
            .field("fb")
            .unwrap()
            .get(0)
            .unwrap()
            .maybe_into()
            .unwrap();
        assert_eq!(b, vec![0i32, 1, 2, 3, 4,]);
    }

    fn save_struct_nested() {
        let mat_a = Mat::maybe_from("fa", 123f64).unwrap();
        let v = vec![0i32, 1, 2, 3, 4];
        let mat_v = Mat::maybe_from("fb", &v).unwrap();

        let data = vec![mat_a, mat_v];
        let nested = Mat::maybe_from("s", data).unwrap();

        let mat_a = Mat::maybe_from("fa", 1234f64).unwrap();
        let v = vec![0i32, 1, 2, 3, 4, 5];
        let mat_v = Mat::maybe_from("fb", &v).unwrap();

        let data = vec![mat_a, mat_v, nested];
        let mat_struct = Mat::maybe_from("s", data).unwrap();

        let mat_file = MatFile::save(root().join("struct-nested.mat")).unwrap();
        mat_file.write(mat_struct);
    }
    #[test]
    fn test_struct_nested() {
        save_struct_nested();
        let mat_file = MatFile::load(root().join("struct-nested.mat")).unwrap();
        let mat: Mat = mat_file.var("s").unwrap();
        let a: f64 = mat
            .field("fa")
            .unwrap()
            .get(0)
            .unwrap()
            .maybe_into()
            .unwrap();
        assert_eq!(a, 1234f64);
        let b: Vec<i32> = mat
            .field("fb")
            .unwrap()
            .get(0)
            .unwrap()
            .maybe_into()
            .unwrap();
        assert_eq!(b, vec![0i32, 1, 2, 3, 4, 5]);
        let v = mat.field("s").unwrap();
        let s = v.get(0).unwrap();
        let a: f64 = s.field("fa").unwrap().get(0).unwrap().maybe_into().unwrap();
        assert_eq!(a, 123f64);
        let b: Vec<i32> = s.field("fb").unwrap().get(0).unwrap().maybe_into().unwrap();
        assert_eq!(b, vec![0i32, 1, 2, 3, 4]);
    }

    fn save_struct_array() {
        let n = 5;
        let mat_a = Box::new((1..=n).map(|i| Mat::maybe_from("fa", i).unwrap()))
            as Box<dyn Iterator<Item = Mat>>;
        let mat_v = Box::new((0..n).map(|_| Mat::maybe_from("fb", vec![0i32, 1, 2, 3, 4]).unwrap()))
            as Box<dyn Iterator<Item = Mat>>;
        let data = vec![mat_a, mat_v];
        let mat_struct = Mat::maybe_from("s", data).unwrap();

        let mat_file = MatFile::save(root().join("struct-array.mat")).unwrap();
        mat_file.write(mat_struct);
    }

    #[test]
    fn test_struct_array() {
        save_struct_array();
        let mat_file = MatFile::load(root().join("struct-array.mat")).unwrap();
        let mat: Mat = mat_file.var("s").unwrap();
        let mat_a = mat.field("fa").unwrap();
        let a = mat_a
            .iter()
            .map(|a| a.maybe_into().unwrap())
            .collect::<Vec<i32>>();
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
        let mat_b = mat.field("fb").unwrap();
        let b = mat_b
            .iter()
            .map(|a| a.maybe_into().unwrap())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(b, vec![vec![0, 1, 2, 3, 4]; 5]);
    }
}

/* #[cfg(test)]
mod tests {

    #[cfg(feature = "nalgebra")]
    #[test]
    fn test_vector() {
        let mat_file = MatFile::load(root().join("arrays.mat")).unwrap();
        let mat: MatVar<Vec<f64>> = mat_file.read("a").unwrap();
        let a: nalgebra::DVector<f64> = mat.into();
        println!("{a}");
        let mat: MatVar<Vec<f64>> = mat_file.read("b").unwrap();
        let b: nalgebra::DVector<f64> = mat.into();
        println!("{b}");
    }

    #[cfg(feature = "nalgebra")]
    #[test]
    fn test_matrix() {
        let mat_file = MatFile::load(root().join("arrays.mat")).unwrap();
        let mat: MatVar<Vec<f64>> = mat_file.read("a").unwrap();
        let a: Option<nalgebra::DMatrix<f64>> = mat.into();
        println!("{:}", a.unwrap());
        let mat: MatVar<Vec<f64>> = mat_file.read("b").unwrap();
        let b: Option<nalgebra::DMatrix<f64>> = mat.into();
        println!("{b:?}");
    }
}
  */
