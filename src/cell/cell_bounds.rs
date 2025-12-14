use super::{Cell, LastCell};

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
