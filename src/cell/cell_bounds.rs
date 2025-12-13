use std::collections::VecDeque;

use super::{Cell, LastCell};
use crate::{Mat, MayBeFrom, Result};

pub trait ToMat<'a, T>
where
    Mat<'a>: MayBeFrom<T>,
{
    fn to_mat(self) -> Result<VecDeque<Mat<'a>>>;
}
impl<'a, T> ToMat<'a, T> for LastCell<T>
where
    Mat<'a>: MayBeFrom<T>,
{
    fn to_mat(self) -> Result<VecDeque<Mat<'a>>> {
        <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), self.item).map(|mat| vec![mat].into())
    }
}
impl<'a, T, C> ToMat<'a, T> for Cell<T, C>
where
    Mat<'a>: MayBeFrom<T> + MayBeFrom<<C as CellBounds>::Item>,
    C: CellBounds + ToMat<'a, <C as CellBounds>::Item>,
{
    fn to_mat(self) -> Result<VecDeque<Mat<'a>>> {
        let mat = <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), self.item)?;
        let mut next_mat = self.next_cell.to_mat()?;
        next_mat.push_front(mat);
        Ok(next_mat)
    }
}

pub trait CellBounds {
    const INDEX: usize;
    type PushBack<T>: CellBounds;
    type Item;
    type NextCell: CellBounds;
    fn push<I>(self, item: I) -> Self::PushBack<I>;
    fn i(&self) -> &Self::Item;
    fn n(&self) -> Option<&Self::NextCell> {
        None
    }
}

impl<T> CellBounds for LastCell<T> {
    const INDEX: usize = 0;
    type PushBack<N> = Cell<T, LastCell<N>>;
    type Item = T;
    type NextCell = Self;

    fn push<I>(self, item: I) -> Self::PushBack<I> {
        Cell {
            item: self.item,
            next_cell: LastCell { item },
        }
    }

    fn i(&self) -> &Self::Item {
        &self.item
    }
}

impl<T, C> CellBounds for Cell<T, C>
where
    C: CellBounds,
{
    const INDEX: usize = <C as CellBounds>::INDEX + 1;
    type PushBack<N> = Cell<T, C::PushBack<N>>;
    type Item = T;
    type NextCell = C;

    fn push<I>(self, item: I) -> Self::PushBack<I> {
        Cell {
            item: self.item,
            next_cell: self.next_cell.push(item),
        }
    }

    fn i(&self) -> &Self::Item {
        &self.item
    }

    fn n(&self) -> Option<&Self::NextCell> {
        Some(&self.next_cell)
    }
}
