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

mod builder;
pub use builder::Builder;
mod matfile;
pub use matfile::{Get, Load, MatFile, Read, Save, Set, SetStruct};
mod matvar;
pub use matvar::MatVar;
mod matstruct;
pub use matstruct::{
    Field, FieldIterator, FieldMatObject, FieldMatObjectIterator, MatStruct, MatStructBuilder,
};
mod mat;

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
    #[error("expected Matlab type {0} found {1}")]
    TryInto(String, String),
    #[error("cannot convert a Matlab array of length {0} into a Rust scalar")]
    Scalar(usize),
}
pub type Result<T> = std::result::Result<T, MatioError>;

/// Interface to Matlab data
pub trait MatObject {
    fn as_mut_ptr(&mut self) -> *mut ffi::matvar_t;
    fn as_ptr(&self) -> *const ffi::matvar_t;
}
pub(crate) trait MatObjectProperty {
    fn rank(&self) -> usize;
    fn dims(&self) -> Vec<u64>;
    fn len(&self) -> usize;
}
impl<T: MatObject> MatObjectProperty for T {
    fn rank(&self) -> usize {
        unsafe { (*self.as_ptr()).rank as usize }
    }
    fn dims(&self) -> Vec<u64> {
        unsafe {
            let n = self.rank();
            Vec::from_raw_parts((*self.as_ptr()).dims, n, n)
        }
    }
    fn len(&self) -> usize {
        self.dims().iter().fold(1, |p, d| p * d) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::{matstruct::MatStructBuilder, Load, MatFile, MatStruct, MatVar, Read, Save};
    use std::path::{Path, PathBuf};

    pub fn root() -> PathBuf {
        Path::new("data").into()
    }

    #[test]
    fn test_load() {
        let _mat_file = MatFile::load(root().join("data.mat")).unwrap();
    }

    #[test]
    fn test_read_scalar() {
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let mat: MatVar<f64> = mat_file.read("a").unwrap();
        let a: f64 = mat.into();
        assert_eq!(a, std::f64::consts::PI);
    }

    #[test]
    fn test_get_scalar() {
        use crate::Get;
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let a: f64 = mat_file.var("a").unwrap();
        assert_eq!(a, std::f64::consts::PI);
    }

    #[test]
    fn test_read_1d() {
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let mat: MatVar<Vec<f64>> = mat_file.read("b").unwrap();
        let b: Vec<f64> = mat.into();
        assert_eq!(b, vec![3f64, 1., 4., 1., 6.])
    }

    #[test]
    fn test_get_1d() {
        use crate::Get;
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let b: Vec<f64> = mat_file.var("b").unwrap();
        assert_eq!(b, vec![3f64, 1., 4., 1., 6.])
    }

    #[test]
    fn test_read_2d() {
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let mat: MatVar<Vec<f64>> = mat_file.read("c").unwrap();
        let c: Vec<f64> = mat.into();
        assert_eq!(c, vec![4f64, 3., 2., 7.])
    }

    #[test]
    fn test_get_2d() {
        use crate::Get;
        let mat_file = MatFile::load(root().join("data.mat")).unwrap();
        let c: Vec<f64> = mat_file.var("c").unwrap();
        assert_eq!(c, vec![4f64, 3., 2., 7.])
    }

    #[test]
    fn test_2d_array() {
        let a = vec![vec![1f64; 3], vec![2f64; 3]];
        let mat_file = MatFile::save(root().join("array.mat")).unwrap();
        let mat: MatVar<Vec<f64>> = MatVar::array(
            "a",
            a.into_iter().flatten().collect::<Vec<f64>>().as_mut_slice(),
            (3, 2),
        )
        .unwrap();
        mat_file.write(mat);
    }

    #[test]
    fn test_save() {
        let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
        {
            let mat_file = MatFile::save(root().join("data.rs.mat")).unwrap();
            mat_file.write(MatVar::<f64>::new("a", &2f64.sqrt()).unwrap());
            mat_file.write(MatVar::<Vec<f64>>::new("b", &mut b).unwrap());
        }
        let mat_file = MatFile::load(root().join("data.rs.mat")).unwrap();
        let mat: MatVar<f64> = mat_file.read("a").unwrap();
        let a: f64 = mat.into();
        assert_eq!(a, 2f64.sqrt());
        let mat: MatVar<Vec<f64>> = mat_file.read("b").unwrap();
        let bb: Vec<f64> = mat.into();
        assert_eq!(b, bb);
    }

    #[test]
    fn test_set() {
        let b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
        {
            let mat_file = MatFile::save(root().join("data.rs.mat")).unwrap();
            <MatFile as crate::Set<f64>>::var(&mat_file, "a", &2f64.sqrt());
            <MatFile as crate::Set<Vec<f64>>>::var(&mat_file, "b", &b);
        }
        use crate::Get;
        let mat_file = MatFile::load(root().join("data.rs.mat")).unwrap();
        let a: f64 = mat_file.var("a").unwrap();
        assert_eq!(a, 2f64.sqrt());
        let bb: Vec<f64> = mat_file.var("b").unwrap();
        assert_eq!(b, bb);
    }

    #[test]
    fn test_save_polytype() {
        let mat_file = MatFile::save(root().join("data-poly.mat")).unwrap();
        mat_file.write(MatVar::<i8>::new("a", &1i8).unwrap());
        mat_file.write(MatVar::<Vec<u16>>::new("c", &mut [3u16; 3]).unwrap());
    }

    #[test]
    fn test_set_polytype() {
        use crate::Set;
        MatFile::save(root().join("data-poly.mat"))
            .unwrap()
            .var("a", &1i8)
            .var("b", &2f32)
            .var("c", &vec![3u16; 3]);
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
        let mat_file = MatFile::save(root().join("struct.mat")).unwrap();
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
        let mat_file = MatFile::save(root().join("struct.mat")).unwrap();
        mat_file.write(mat);
    }

    #[test]
    fn test_struct_property() {
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
        println!("{mat}");
    }
    #[test]
    fn test_save_nested_struct() {
        let mut builder = {
            use crate::Field;
            MatStruct::new("a")
                .field("fa", &10f64)
                .unwrap()
                .field("fb", &vec![0i32, 1, 2, 3])
                .unwrap()
        };
        let nested = {
            use crate::Field;
            MatStruct::new("a")
                .field("fa", &10f64)
                .unwrap()
                .field("fb", &vec![0i32, 1, 2, 3])
                .unwrap()
                .build()
                .unwrap()
        };
        builder = <MatStructBuilder as crate::FieldMatObject<MatStruct>>::field(
            builder, "nested", nested,
        )
        .unwrap();
        let mat_file = MatFile::save(root().join("struct_nested.mat")).unwrap();
        mat_file.write(builder.build().unwrap());
    }

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
