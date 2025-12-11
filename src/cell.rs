// https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15

mod cell;
mod cell_bounds;
mod convert;
mod last_cell;
pub use cell::Cell;
pub use cell_bounds::CellBounds;
pub use last_cell::LastCell;

// pub trait ItemBounds {} // where for <'a> Mat<'a>: MayBeFrom<T> {}
// impl<T> ItemBounds for T where for<'a> Mat<'a>: MayBeFrom<T> {}
#[cfg(test)]
mod tests {
    use crate::{Mat, MatFile, MayBeFrom, MayBeInto};

    use super::*;

    #[test]
    fn maybe_from() {
        let c = Cell::new(1u32).push(1.23456f64).push("qwerty".to_string());
        dbg!(&c);
        println!("{c}");
        let q = c.i();
        let q = c.n().map(|c| c.i());
        let q = c.n().and_then(|c| c.n().map(|c| c.i()));
        let q = c.n().and_then(|c| c.n().and_then(|c| c.n().map(|c| c.i())));
        dbg!(&q);
        // let q = c.n().unwrap().n().unwrap().n().unwrap().i();
        let m: Mat = MayBeFrom::maybe_from("cell", c).unwrap();
        let matf = MatFile::save("cell.mat").unwrap();
        matf.write(m);
        // dbg!(m.len());
    }

    #[test]
    fn maybe_into() {
        let matf = MatFile::load("cell.mat").unwrap();
        let mat = matf.read("cell").unwrap();
        let c: Cell<u32, Cell<f64, LastCell<String>>> = MayBeInto::maybe_into(mat).unwrap();
        dbg!(&c);
        println!("{c}");
    }
}
