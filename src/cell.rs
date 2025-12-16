/*!
Matlab cell array

The module defines a [Cell] structure that is used to read from and to write to
a Matlab cell array.

## Examples

A 3 elements cell array is build and saved to a Mat file with:
```
use matio_rs::{MatFile, cell::{Cell, CellBounds}};
# let file = tempfile::NamedTempFile::new().unwrap();
# let data_path = file.path();
let cell = Cell::new(1u32).push(1.23456f64).push("qwerty".to_string());
MatFile::save(data_path)?
    .var("cell", cell)?;
# Ok::<(), matio_rs::MatioError>(())
```

The same 3 elements cell array is loaded with:
```
use matio_rs::{MatFile, cell::{Cell, LastCell, CellBounds}};
# let file = tempfile::NamedTempFile::new().unwrap();
# let data_path = file.path();
# let cell = Cell::new(1u32).push(1.23456f64).push("qwerty".to_string());
# MatFile::save(data_path)?
#    .var("cell", cell)?;
let c: Cell<u32, Cell<f64, LastCell<String>>> = MatFile::load("cell.mat")?.var("cell")?;
# Ok::<(), matio_rs::MatioError>(())
```

Tuples are written to and read from Matlab cell array and often are a more
convenient interface that using [Cell] directly.
The 2 examples above could be replaced with:
```
use matio_rs::MatFile;
# let file = tempfile::NamedTempFile::new().unwrap();
# let data_path = file.path();
let cell = (1u32, 1.23456f64, "qwerty");
MatFile::save(data_path)?
    .var("cell", cell)?;
# Ok::<(), matio_rs::MatioError>(())
```
and:
```
use matio_rs::MatFile;
# let file = tempfile::NamedTempFile::new().unwrap();
# let data_path = file.path();
# let cell = (1u32, 1.23456f64, "qwerty");
# MatFile::save(data_path)?
#    .var("cell", cell)?;
let c: (u32, f64, String) = MatFile::load("cell.mat")?.var("cell")?;
# Ok::<(), matio_rs::MatioError>(())
```
*/

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
        let c = Cell::new(1u32).push(1.23456f64).push("qwerty".to_string());
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
        matf.var("cell", c.clone()).unwrap();

        let matf = MatFile::load("cell.mat").unwrap();
        let c1: Cell<u32, Cell<f64, LastCell<String>>> = matf.var("cell").unwrap();
        dbg!(&c1);
        println!("{c1}");
        assert_eq!(c, c1);
    }
}
