use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use super::{CellBounds, LastCell};
use crate::{Mat, MatioError, MayBeFrom, MayBeInto};

pub struct Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    C: CellBounds,
{
    pub(super) item: T,
    pub(super) next_cell: C,
}

impl<T: Debug, C> Debug for Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    C: CellBounds + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("item", &self.item)
            .field("next_cell", &self.next_cell)
            .finish()
    }
}

impl<T: Display, C> Display for Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    C: CellBounds + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = <Self as CellBounds>::INDEX;
        writeln!(f, "{i}: {}", self.item)?;
        writeln!(f, "{}", self.next_cell)
    }
}

impl<'a, T, C> TryFrom<Cell<T, C>> for VecDeque<Mat<'a>>
where
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
    C: CellBounds,
{
    type Error = MatioError;

    fn try_from(cell: Cell<T, C>) -> std::result::Result<Self, Self::Error> {
        let mat = <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), cell.item)?;
        let mut next_mat = cell.next_cell.to_mat()?;
        next_mat.push_front(mat);
        Ok(next_mat)
    }
}

impl<T> Cell<T, LastCell<T>>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    pub fn new(item: T) -> LastCell<T> {
        LastCell { item }
    }
}
