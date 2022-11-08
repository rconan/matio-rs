//! Convert CFD opds.bin files into opds.mat file

use std::fs::File;

use matio_rs::{Field, MatFile, MatStruct, Save};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Opds {
    pub values: Vec<f64>,
    pub mask: Vec<bool>,
}

fn main() -> anyhow::Result<()> {
    let data: Opds = bincode::deserialize_from(File::open("data/opds.bin")?)?;
    let mat = MatStruct::new("opds")
        .field("values", &data.values)?
        .field(
            "mask",
            &data.mask.into_iter().map(|x| x as u8).collect::<Vec<u8>>(),
        )?
        .build()?;
    let mat_file = MatFile::save("data/opds.mat")?;
    mat_file.write(mat);
    Ok(())
}
