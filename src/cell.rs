// https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15

mod cell;
mod cell_bounds;
mod cellvec;
mod convert;
mod last_cell;

pub use cell::Cell;
pub use cell_bounds::CellBounds;
pub use cellvec::CellVec;
pub use last_cell::LastCell;

// pub trait ItemBounds {} // where for <'a> Mat<'a>: MayBeFrom<T> {}
// impl<T> ItemBounds for T where for<'a> Mat<'a>: MayBeFrom<T> {}
#[cfg(test)]
mod tests {
    use crate::MatFile;

    use super::*;

    #[test]
    fn cell() {
        let c = Cell::new(1u32).push(1.23456f64).push("qwerty");
        dbg!(&c);
        println!("{c}");
        let _q = c.i();
        let q = c.n().map(|c| c.i());
        assert!(q.is_some());
        let q = c.n().and_then(|c| c.n().map(|c| c.i()));
        assert!(q.is_some());
        let q = c.n().and_then(|c| c.n().and_then(|c| c.n().map(|c| c.i())));
        assert!(q.is_none());
        dbg!(&q);

        let matf = MatFile::save("cell.mat").unwrap();
        matf.var("cell", c).unwrap();

        let matf = MatFile::load("cell.mat").unwrap();
        let c1: Cell<u32, Cell<f64, LastCell<String>>> = matf.var("cell").unwrap();
        dbg!(&c1);
        println!("{c1}");
        // assert_eq!(c,c1);
    }

}
