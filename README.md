Rust wrapper to [MATLAB MAT file I/O library](https://github.com/tbeu/matio)


This crate provides bindings and wrappers for [MATIO](https://github.com/tbeu/matio):
MATLAB MAT file I/O C library

## Examples

Loading a mat file
```rust
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
```
Reading a scalar Matlab variable: a = π
```rust
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
if let Ok(mat) = mat_file.read("a") {
    println!("{mat}");
    let a: f64 = mat.into();
    println!("{a:?}");
}
```
Reading a Matlab vector: b = [3.0, 1.0, 4.0, 1.0, 6.0]
```rust
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
if let Ok(mat) = mat_file.read("b") {
    println!("{mat}");
    let b: Vec<f64> = mat.into();
    println!("{b:?}");
}
```
Reading a Matlab array: c = [4, 2; 3, 7]
```rust
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
if let Ok(mat) = mat_file.read("c") {
    println!("{mat}");
    let c: Vec<f64> = mat.into();
    println!("{c:?}");
}
```
Saving to a mat file
```rust
use matio_rs::{MatFile, MatVar, Save};
let mat_file = MatFile::save("data.rs.mat")?;
let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
mat_file.write(MatVar::<f64>::new("a", 2f64.sqrt())?)
        .write(MatVar::<Vec<f64>>::new("b", &mut b)?);
```
