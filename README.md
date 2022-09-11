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
let a: f64 = MatFile::load(data_path)?.var("a")?;
println!("{a:?}");
```
Reading a Matlab vector: b = [3.0, 1.0, 4.0, 1.0, 6.0]
```rust
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
let b: Vec<f64> = MatFile::load(data_path)?.var("b")?;
println!("{b:?}");
```
Reading a Matlab array: c = [4, 2; 3, 7]
```rust
use matio_rs::{MatFile, Load};
let mat_file = MatFile::load("data.mat")?;
let c: Vec<f64> = MatFile::load(data_path)?.var("c")?;
println!("{c:?}");
```
Saving to a mat file
```rust
use matio_rs::{MatFile, MatVar, Save};
let mat_file = MatFile::save("data.rs.mat")?;
let mut b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
MatFile::save(data_path)?
    .var("a", &2f64.sqrt())
    .var("b", &b);
```
Writing a Matlab structure to a mat file
```rust
use matio_rs::{MatFile, MatStruct, Save, Field};
let mut mat = MatStruct::new("s", vec!["fa", "fb"])?
            .field("fa", &123f64)?
            .field("fb", &vec![0i32, 1, 2, 3, 4])?;
let mat_file = MatFile::save("struct.mat")?;
mat_file.write(mat);
```
Writing a Matlab structure array to a mat file
```rust
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
```