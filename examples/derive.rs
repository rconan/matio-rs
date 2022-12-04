use matio_rs::{MatFile, MatIO};
use tempfile::NamedTempFile;

#[derive(Debug, Default, MatIO)]
struct SMat {
    a: f64,
    b: Vec<u32>,
    s: Nested,
}
#[derive(Debug, Default, MatIO)]
struct Nested {
    a: f64,
    b: Vec<u32>,
}

fn main() -> anyhow::Result<()> {
    let n = Nested {
        a: 1f64,
        b: vec![2, 3, 4, 5],
    };
    let a = SMat {
        a: 1f64,
        b: vec![2, 3, 4, 5],
        s: n,
    };

    let file = NamedTempFile::new().unwrap();
    MatFile::save(&file)?.var("a", &a)?;
    let aa: SMat = MatFile::load(file)?.var("a")?;
    dbg!(aa);

    Ok(())
}
