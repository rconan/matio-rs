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
