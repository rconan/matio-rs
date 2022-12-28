use matio_rs::*;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub fn root() -> PathBuf {
    //Path::new("data").into()
    let file = NamedTempFile::new().unwrap();
    file.path().to_path_buf()
}

#[test]
fn test_read_write_scalar() {
    let path = root();
    let mat_file = MatFile::save(&path).unwrap();
    mat_file.var("a", std::f64::consts::PI).unwrap();
    let mat_file = MatFile::load(&path).unwrap();
    let a: f64 = mat_file.var("a").unwrap();
    assert_eq!(a, std::f64::consts::PI);
}

#[test]
fn test_read_1d() {
    let path = root();
    let mat_file = MatFile::save(&path).unwrap();
    mat_file.var("b", vec![3f64, 1., 4., 1., 6.]).unwrap();
    let mat_file = MatFile::load(&path).unwrap();
    let b: Vec<f64> = mat_file.var("b").unwrap();
    assert_eq!(b, vec![3f64, 1., 4., 1., 6.])
}

#[test]
fn test_get_2d() {
    let path = root();
    let mat_file = MatFile::save(&path).unwrap();
    mat_file.var("c", vec![4f64, 3., 2., 7.]).unwrap();
    let mat_file = MatFile::load(&path).unwrap();
    let c: Vec<f64> = mat_file.var("c").unwrap();
    assert_eq!(c, vec![4f64, 3., 2., 7.])
}

#[test]
fn test_readwrite() {
    let path = root();
    let b = (0..5).map(|x| (x as f64).cosh()).collect::<Vec<f64>>();
    MatFile::save(&path)
        .unwrap()
        .var("a", 2f64.sqrt())
        .unwrap()
        .var("b", &b)
        .unwrap();
    let mat_file = MatFile::load(&path).unwrap();
    let a: f64 = mat_file.var("a").unwrap();
    assert_eq!(a, 2f64.sqrt());
    let bb: Vec<f64> = mat_file.var("b").unwrap();
    assert_eq!(b, bb);
}

fn polytype(path: &PathBuf) {
    MatFile::save(path)
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
    let path = root();
    polytype(&path);
    let mat_file = MatFile::load(&path).unwrap();
    let a: i8 = mat_file.var("a").unwrap();
    assert_eq!(a, 1i8);
    let b: f32 = mat_file.var("b").unwrap();
    assert_eq!(b, 2f32);
    let c: Vec<u16> = mat_file.var("c").unwrap();
    assert_eq!(c, vec![3u16; 3]);
}

fn save_struct(path: &PathBuf) {
    let mat_a = Mat::maybe_from("fa", 123f64).unwrap();
    let v = vec![0i32, 1, 2, 3, 4];
    let mat_v = Mat::maybe_from("fb", &v).unwrap();

    let data = vec![mat_a, mat_v];
    let mat_struct = Mat::maybe_from("s", data).unwrap();

    let mat_file = MatFile::save(path).unwrap();
    mat_file.write(mat_struct);
}

#[test]
fn test_struct() {
    let path = root();
    save_struct(&path);
    let mat_file = MatFile::load(&path).unwrap();
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

fn save_struct_nested(path: &PathBuf) {
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

    let mat_file = MatFile::save(path).unwrap();
    mat_file.write(mat_struct);
}
#[test]
fn test_struct_nested() {
    let path = root();
    save_struct_nested(&path);
    let mat_file = MatFile::load(&path).unwrap();
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

fn save_struct_array(path: &PathBuf) {
    let n = 5;
    let mat_a = Box::new((1..=n).map(|i| Mat::maybe_from("fa", i).unwrap()))
        as Box<dyn Iterator<Item = Mat>>;
    let mat_v = Box::new((0..n).map(|_| Mat::maybe_from("fb", vec![0i32, 1, 2, 3, 4]).unwrap()))
        as Box<dyn Iterator<Item = Mat>>;
    let data = vec![mat_a, mat_v];
    let mat_struct = Mat::maybe_from("s", data).unwrap();

    let mat_file = MatFile::save(path).unwrap();
    mat_file.write(mat_struct);
}

#[test]
fn test_struct_array() {
    let path = root();
    save_struct_array(&path);
    let mat_file = MatFile::load(&path).unwrap();
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

#[cfg(feature = "nalgebra")]
mod nalgebra_matio {
    use super::*;
    #[test]
    fn test_nalgebra_vector() {
        let na_v = nalgebra::DVector::from_iterator(5, 0..5);
        let path = root();
        MatFile::save(&path).unwrap().var("na_v", &na_v).unwrap();
        let v: nalgebra::DMatrix<i32> = MatFile::load(path).unwrap().var("na_v").unwrap();
        assert_eq!(na_v, v);
    }

    #[test]
    fn test_nalgebra_matrix() {
        let na_m = nalgebra::DMatrix::from_iterator(3, 2, 0..6);
        let path = root();
        MatFile::save(&path).unwrap().var("na_m", &na_m).unwrap();
        let m: nalgebra::DMatrix<i32> = MatFile::load(path).unwrap().var("na_m").unwrap();
        assert_eq!(na_m, m);
    }
}
