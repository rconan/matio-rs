use matio_rs::*;
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
