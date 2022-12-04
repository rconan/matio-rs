use matio_rs::{Mat, MatFile, MayBeFrom, MayBeInto};
use tempfile::NamedTempFile;

#[derive(Debug, Default)]
struct SMat {
    a: f64,
    b: Vec<u32>,
    s: Nested,
}
#[derive(Debug, Default, Clone)]
struct Nested {
    a: f64,
    b: Vec<u32>,
}

impl<'a> MayBeFrom<&Nested> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &Nested) -> matio_rs::Result<Self>
    where
        Self: Sized,
    {
        let mats: Vec<Mat> = vec![
            Mat::maybe_from("a", data.a)?,
            Mat::maybe_from("b", &data.b)?,
        ];
        MayBeFrom::maybe_from(name, mats)
    }
}

impl<'a> MayBeInto<Nested> for &Mat<'a> {
    fn maybe_into(self) -> matio_rs::Result<Nested> {
        Ok(Nested {
            a: self.field("a")?.get(0).unwrap().maybe_into()?,
            b: self.field("b")?.get(0).unwrap().maybe_into()?,
        })
    }
}

impl<'a> MayBeFrom<&SMat> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &SMat) -> matio_rs::Result<Self>
    where
        Self: Sized,
    {
        let mats: Vec<Mat> = vec![
            Mat::maybe_from("a", data.a)?,
            Mat::maybe_from("b", &data.b)?,
            Mat::maybe_from("s", &data.s)?,
        ];
        MayBeFrom::maybe_from(name, mats)
    }
}
impl<'a> MayBeInto<SMat> for Mat<'a> {
    fn maybe_into(self) -> matio_rs::Result<SMat> {
        Ok(SMat {
            a: self.field("a")?.get(0).unwrap().maybe_into()?,
            b: self.field("b")?.get(0).unwrap().maybe_into()?,
            s: self.field("s")?.get(0).unwrap().maybe_into()?,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let n = Nested {
        a: 1f64,
        b: vec![2, 3, 4, 5],
    };
    let a = SMat {
        a: 1f64,
        b: vec![2, 3, 4, 5],
        s: n.clone(),
    };

    let file = NamedTempFile::new().unwrap();
    MatFile::save(&file)?.var("a", &a)?;
    let aa: SMat = MatFile::load(file)?.var("a")?;
    dbg!(aa);

    Ok(())
}
