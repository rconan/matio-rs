use std::vec;

use super::{Cell, CellBounds, LastCell};

/// Cell of the same item type
pub trait CellVec<T>: CellBounds<Item = T> {
    /// Returns a reference to the item of cell with index `idx`
    ///
    /// Index `0` corresponds to the last cell
    fn get(&self, idx: usize) -> Option<&T>;
    /// Converts a [Cell] into a [Vec]
    fn into_vec(self) -> Vec<T>;
    /// Empties a [Vec] into a [Cell]
    fn from_vec(v: &mut Vec<T>) -> Option<Self>
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

    fn into_vec(self) -> Vec<T> {
        vec![self.item]
    }

    fn from_vec(v: &mut Vec<T>) -> Option<Self> {
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

    fn into_vec(self) -> Vec<T> {
        let mut v = vec![self.item];
        let nv = <C as CellVec<T>>::into_vec(self.next_cell);
        v.extend(nv);
        v
    }

    fn from_vec(v: &mut Vec<T>) -> Option<Self> {
        let Some(item) = v.pop() else { return None };
        if let Some(next_cell) = <C as CellVec<T>>::from_vec(v) {
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
        <Cell<T, C> as CellVec<T>>::into_vec(cell)
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
        <Self as CellVec<T>>::from_vec(&mut v).unwrap()
    }
}

impl<T, C> FromIterator<T> for Cell<T, C>
where
    C: CellVec<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v: Vec<T> = iter.into_iter().collect();
        <Self as CellVec<T>>::from_vec(&mut v).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn from_vec() {
    //     let v: Vec<_> = (0i32..5).collect();
    //     let c: Cell<_, Cell<_,_>> = Cell::<i32, _>::from(v);
    // }

    #[test]
    fn into_vec() {
        let c = Cell::new(0i32).push(1i32).push(2i32).push(3i32).push(4i32);
        let v: Vec<i32> = c.into();
        assert_eq!(v, vec![0i32, 1, 2, 3, 4]);
    }
}
