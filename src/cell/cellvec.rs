use std::vec;

use super::{Cell, CellBounds, LastCell};

pub trait CellVec<T>: CellBounds<Item = T> {
    fn get(&self, idx: usize) -> Option<&T>;
    fn into(self) -> Vec<T>;
    fn from(v: &mut Vec<T>) -> Option<Self>
    where
        Self: Sized;
}

impl<T> CellVec<T> for LastCell<T> {
    fn get(&self, idx: usize) -> Option<&T> {
        if idx == <Self as CellBounds>::INDEX {
            Some(&self.item)
        } else {
            None
        }
    }

    fn into(self) -> Vec<T> {
        vec![self.item]
    }

    fn from(v: &mut Vec<T>) -> Option<Self> {
        if let Some(item) = v.pop() {
            Some(Self { item })
        } else {
            None
        }
    }
}

impl<T, C> CellVec<T> for Cell<T, C>
where
    C: CellVec<T>,
{
    fn get(&self, idx: usize) -> Option<&T> {
        if idx == <Self as CellBounds>::INDEX {
            Some(&self.item)
        } else {
            <C as CellVec<T>>::get(&self.next_cell, idx)
        }
    }

    fn into(self) -> Vec<T> {
        let mut v = vec![self.item];
        let nv = <C as CellVec<T>>::into(self.next_cell);
        v.extend(nv);
        v
    }

    fn from(v: &mut Vec<T>) -> Option<Self> {
        let Some(item) = v.pop() else { return None };
        if let Some(next_cell) = <C as CellVec<T>>::from(v) {
            Some(Self { item, next_cell })
        } else {
            None
        }
    }
}

impl<T, C> From<Cell<T, C>> for Vec<T>
where
    C: CellVec<T>,
{
    fn from(cell: Cell<T, C>) -> Self {
        <Cell<T, C> as CellVec<T>>::into(cell)
    }
}

impl<T, C> IntoIterator for Cell<T, C>
where
    C: CellVec<T>,
{
    type Item = T;

    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<T>::from(self).into_iter()
    }
}

impl<T, C> From<Vec<T>> for Cell<T, C>
where
    C: CellVec<T>,
{
    fn from(mut v: Vec<T>) -> Self {
        <Self as CellVec<T>>::from(&mut v).unwrap()
    }
}

impl<T, C> FromIterator<T> for Cell<T, C>
where
    C: CellVec<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v: Vec<T> = iter.into_iter().collect();
        <Self as CellVec<T>>::from(&mut v).unwrap()
    }
}
