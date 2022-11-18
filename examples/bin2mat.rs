//! Convert CFD opds.bin files into opds.mat file

use std::fs::File;

use matio_rs::{Mat, MatFile, MatTryFrom};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Opds {
    pub values: Vec<f64>,
    pub mask: Vec<bool>,
}

fn main() -> anyhow::Result<()> {
    let data: Opds = bincode::deserialize_from(File::open("data/opds.bin")?)?;
    let mask = data.mask.into_iter().map(|x| x as u8).collect::<Vec<u8>>();
    let mat_data = vec![
        Mat::maybe_from("values", &data.values)?,
        Mat::maybe_from("mask", &mask)?,
    ];
    let mat_struct = Mat::maybe_from("opd", mat_data)?;
    let mat_file = MatFile::save("data/opds.mat")?;
    mat_file.write(mat_struct);
    Ok(())
}
