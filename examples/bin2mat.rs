//! Convert CFD opds.bin files into opds.mat file

use std::fs::File;

use matio_rs::{Mat, MatFile, MayBeFrom, MayBeInto};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Opds {
    pub values: Vec<f64>,
    pub mask: Vec<bool>,
}

impl<'a> MayBeFrom<&'a Opds> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &'a Opds) -> matio_rs::Result<Self> {
        let m: Vec<u8> = data
            .mask
            .iter()
            .map(|&m| if m { 1u8 } else { 0u8 })
            .collect();
        let mats = vec![
            Mat::maybe_from("values", &data.values)?,
            Mat::maybe_from("mask", m)?,
        ];
        MayBeFrom::maybe_from(name, mats)
    }
}

impl<'a> MayBeInto<Opds> for Mat<'a> {
    fn maybe_into(self) -> matio_rs::Result<Opds> {
        let m: Vec<u8> = self.field("values")?.get(0).unwrap().maybe_into()?;
        Ok(Opds {
            values: self.field("values")?.get(0).unwrap().maybe_into()?,
            mask: m
                .into_iter()
                .map(|m| if m == 1 { true } else { false })
                .collect(),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let data: Opds = bincode::deserialize_from(File::open("data/opds.bin")?)?;
    MatFile::save("data/opds.mat")?.var("opds", &data)?;
    Ok(())
}
