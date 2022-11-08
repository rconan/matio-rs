use matio_rs::{MatFile, MatStruct, Save};
use matio_rs_derive::MatStruct;

#[derive(Default, Debug, MatStruct)]
pub struct A {
    a: f64,
    b: u8,
    c: Vec<u32>,
    aa: AA,
}

#[derive(Default, Debug, MatStruct)]
pub struct AA {
    a: f32,
    aaa: AAA,
}

#[derive(Default, Debug, MatStruct)]
pub struct AAA {
    a: Vec<u8>,
}

fn main() {
    let mat: MatStruct = {
        let data = A {
            a: 1.234,
            b: 1,
            c: vec![1, 2, 3],
            aa: AA {
                a: 123456789f32,
                aaa: AAA { a: vec![1; 8] },
            },
        };
        (&data).into()
    };

    let matfile = MatFile::save("astruct.mat").unwrap();
    matfile.write(mat);
}
