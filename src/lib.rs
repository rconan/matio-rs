/*!
# Rust bindings and wrappers for [MATIO](https://github.com/tbeu/matio)

This crate provides bindings and wrappers for [MATIO](https://github.com/tbeu/matio):
MATLAB MAT file I/O C library

## Examples

Saving to a Mat file
```
use matio_rs::MatFile;
# let file = tempfile::NamedTempFile::new().unwrap();
# let data_path = file.path();
MatFile::save(data_path)?
    .var("a", 1i8)?
    .var("b", 2f32)?
    .var("c", &vec![3u16; 3])?;
# Ok::<(), matio_rs::MatioError>(())
```
and then loading the data back into Rust
```
# use matio_rs::MatFile;
# let file = tempfile::NamedTempFile::new().unwrap();
# let data_path = file.path();
# MatFile::save(data_path)?
#   .var("a", 1i8)?
#    .var("b", 2f32)?
#    .var("c", &vec![3u16; 3])?;
let mat_file = MatFile::load(data_path)?;
let a: i8 = mat_file.var("a")?;
let b: f32 = mat_file.var("b")?;
let c: Vec<u16> = mat_file.var("c")?;
# Ok::<(), matio_rs::MatioError>(())
```

Saving data to a Matlab structure
```
use matio_rs::{MatFile, Mat, MayBeFrom};
# let file = tempfile::NamedTempFile::new()?;
# let data_path = file.path();
let mat_a = Mat::maybe_from("fa", 123f64)?;
let b = vec![0i32, 1, 2, 3, 4];
let mat_v = Mat::maybe_from("fb", &b)?;
let data = vec![mat_a, mat_v];
let mat_struct = Mat::maybe_from("s", data)?;
let mat_file = MatFile::save(data_path)?;
mat_file.write(mat_struct);
# Ok::<(), matio_rs::MatioError>(())
```
and then loading the structure fields back into Rust variables
```
use matio_rs::{MatFile, Mat, MayBeInto};
# use matio_rs::{MayBeFrom};
# let file = tempfile::NamedTempFile::new()?;
# let data_path = file.path();
# let mat_a = Mat::maybe_from("fa", 123f64)?;
# let b = vec![0i32, 1, 2, 3, 4];
# let mat_v = Mat::maybe_from("fb", &b)?;
# let data = vec![mat_a, mat_v];
# let mat_struct = Mat::maybe_from("s", data)?;
# let mat_file = MatFile::save(data_path)?;
# mat_file.write(mat_struct);
let mat_file = MatFile::load(&data_path)?;
let mat: Mat = mat_file.var("s")?;
let a: f64 = mat
    .field("fa")?
    .get(0).unwrap()
    .maybe_into()?;
let b: Vec<i32> = mat
    .field("fb")?
    .get(0).unwrap()
    .maybe_into()?;
# Ok::<(), matio_rs::MatioError>(())
```

Rust structure with the [MatIO] derive attribute can be dispatched like any other variables:
```
use matio_rs::{MatFile, MatIO};
# use tempfile::NamedTempFile;

#[derive(Debug, Default, MatIO)]
struct SMat {
    a: f64,
    b: Vec<u32>,
    s: Nested,
}
#[derive(Debug, Default, MatIO)]
struct Nested {
    a: f64,
    b: Vec<u32>,
}
let n = Nested {
    a: 1f64,
    b: vec![2, 3, 4, 5],
};
let a = SMat {
    a: 1f64,
    b: vec![2, 3, 4, 5],
    s: n,
};
# let file = NamedTempFile::new().unwrap();
MatFile::save(&file)?.var("a", &a)?;
let aa: SMat = MatFile::load(file)?.var("a")?;
# Ok::<(), matio_rs::MatioError>(())
```

[nalgebra](https://docs.rs/nalgebra/latest/nalgebra/) vectors and matrices can be read from and
 written to Mat files providing the `nalgebra` feature
```
use matio_rs::MatFile;
# use tempfile::NamedTempFile;
# let file = NamedTempFile::new().unwrap();
let na_v = nalgebra::DVector::from_iterator(5, 0..5);
MatFile::save(&file).unwrap().var("na_v", &na_v).unwrap();
let v: nalgebra::DMatrix<i32> = MatFile::load(file).unwrap().var("na_v").unwrap();
```
```
use matio_rs::MatFile;
# use tempfile::NamedTempFile;
# let file = NamedTempFile::new().unwrap();
let na_m = nalgebra::DMatrix::from_iterator(3, 2, 0..6);
MatFile::save(&file).unwrap().var("na_m", &na_m).unwrap();
let m: nalgebra::DMatrix<i32> = MatFile::load(file).unwrap().var("na_m").unwrap();
```
*/

use std::io;
use thiserror::Error;

// mod builder;
// pub use builder::Builder;
mod matfile;
pub use matfile::{MatFile, MatFileRead, MatFileWrite};
mod datatype;
pub(crate) use datatype::{DataType, MatType};
mod mat;
pub use mat::Mat;
mod convert;
pub use convert::{MayBeFrom, MayBeInto};
pub use derive::MatIO;

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
    #[error("expected rank 2, found {0}")]
    Rank(usize),
}
pub type Result<T> = std::result::Result<T, MatioError>;
