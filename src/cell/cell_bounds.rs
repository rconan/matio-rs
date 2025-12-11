use std::collections::VecDeque;

use super::{Cell, LastCell};
use crate::{Mat, MayBeFrom, MayBeInto, Result};

pub trait CellBounds {
    const INDEX: usize;
    type PushBack<T>: CellBounds
    where
        for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>;
    type Item;
    type NextCell: CellBounds;
    fn push<I>(self, item: I) -> Self::PushBack<I>
    where
        for<'a> Mat<'a>: MayBeFrom<I> + MayBeInto<I>;
    fn to_mat<'a>(self) -> Result<VecDeque<Mat<'a>>>;
    fn i(&self) -> &Self::Item;
    fn n(&self) -> Option<&Self::NextCell> {
        None
    }
}

impl<T> CellBounds for LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    const INDEX: usize = 0;
    type PushBack<N>
        = Cell<T, LastCell<N>>
    where
        for<'a> Mat<'a>: MayBeFrom<N> + MayBeInto<N>;
    type Item = T;
    type NextCell = Self;

    fn push<I>(self, item: I) -> Self::PushBack<I>
    where
        for<'a> Mat<'a>: MayBeFrom<I> + MayBeInto<I>,
    {
        Cell {
            item: self.item,
            next_cell: LastCell { item },
        }
    }

    fn to_mat<'a>(self) -> Result<VecDeque<Mat<'a>>> {
        <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), self.item).map(|mat| vec![mat].into())
    }

    fn i(&self) -> &Self::Item {
        &self.item
    }
}

impl<T, C> CellBounds for Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    C: CellBounds,
{
    const INDEX: usize = <C as CellBounds>::INDEX + 1;
    type PushBack<N>
        = Cell<T, C::PushBack<N>>
    where
        for<'a> Mat<'a>: MayBeFrom<N> + MayBeInto<N>;
    type Item = T;
    type NextCell = C;

    fn push<I>(self, item: I) -> Self::PushBack<I>
    where
        for<'a> Mat<'a>: MayBeFrom<I> + MayBeInto<I>,
    {
        Cell {
            item: self.item,
            next_cell: self.next_cell.push(item),
        }
    }

    fn to_mat<'a>(self) -> Result<VecDeque<Mat<'a>>> {
        let mat = <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), self.item)?;
        let mut next_mat = self.next_cell.to_mat()?;
        next_mat.push_front(mat);
        Ok(next_mat)
    }

    fn i(&self) -> &Self::Item {
        &self.item
    }

    fn n(&self) -> Option<&Self::NextCell> {
        Some(&self.next_cell)
    }
}
