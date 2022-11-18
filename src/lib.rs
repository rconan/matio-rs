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
pub use convert::{MayBeFrom, MayBeInto};

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
