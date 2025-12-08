use std::{collections::VecDeque, ffi::CString};

use crate::{Mat, MatioError, MayBeFrom, MayBeInto, Result};

// https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15

pub struct Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T>,
    C: CellBounds,
{
    item: T,
    next_cell: C,
}

pub struct LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T>,
{
    item: T,
}

// pub trait ItemBounds {} // where for <'a> Mat<'a>: MayBeFrom<T> {}
// impl<T> ItemBounds for T where for<'a> Mat<'a>: MayBeFrom<T> {}

pub trait CellBounds {
    type PushBack<T>: CellBounds
    where
        for<'a> Mat<'a>: MayBeFrom<T>;
    type Item;
    type NextCell: CellBounds;
    fn push<I>(self, item: I) -> Self::PushBack<I>
    where
        for<'a> Mat<'a>: MayBeFrom<I>;
    fn to_mat<'a>(self) -> Result<VecDeque<Mat<'a>>>;
}

impl<T> CellBounds for LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T>,
{
    type PushBack<N>
        = Cell<T, LastCell<N>>
    where
        for<'a> Mat<'a>: MayBeFrom<N>;
    type Item = T;
    type NextCell = Self;

    fn push<I>(self, item: I) -> Self::PushBack<I>
    where
        for<'a> Mat<'a>: MayBeFrom<I>,
    {
        Cell {
            item: self.item,
            next_cell: LastCell { item },
        }
    }

    fn to_mat<'a>(self) -> Result<VecDeque<Mat<'a>>> {
        <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), self.item).map(|mat| vec![mat].into())
    }
}

impl<'a, T> TryFrom<LastCell<T>> for VecDeque<Mat<'a>>
where
    for<'b> Mat<'b>: MayBeFrom<T>,
{
    type Error = MatioError;

    fn try_from(last_cell: LastCell<T>) -> std::result::Result<Self, Self::Error> {
        let mat = <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), last_cell.item);
        mat.map(|mat| vec![mat].into())
    }
}

impl<T, C> CellBounds for Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T>,
    C: CellBounds,
{
    type PushBack<N>
        = Cell<T, C::PushBack<N>>
    where
        for<'a> Mat<'a>: MayBeFrom<N>;
    type Item = T;
    type NextCell = C;

    fn push<I>(self, item: I) -> Self::PushBack<I>
    where
        for<'a> Mat<'a>: MayBeFrom<I>,
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
}

impl<'a, T, C> TryFrom<Cell<T, C>> for VecDeque<Mat<'a>>
where
    for<'b> Mat<'b>: MayBeFrom<T>,
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
    for<'a> Mat<'a>: MayBeFrom<T>,
{
    pub fn new(item: T) -> LastCell<T> {
        LastCell { item }
    }
}

impl<'a, T, C> MayBeFrom<Cell<T, C>> for Mat<'a>
where
    C: CellBounds,
    for<'b> Mat<'b>: MayBeFrom<T>,
{
    fn maybe_from<S: Into<String>>(name: S, cell: Cell<T, C>) -> Result<Self> {
        let mut mat = VecDeque::try_from(cell)?;
        mat.iter_mut().for_each(|mat| {
            mat.as_ref = true;
        });
        let c_name = CString::new(name.into())?;
        let mut dims = [1, mat.len()];
        let matcell_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                ffi::matio_classes_MAT_C_CELL,
                ffi::matio_types_MAT_T_CELL,
                2,
                dims.as_mut_ptr(),
                std::ptr::null_mut(),
                0,
            )
        };
        for (i, mat) in mat.into_iter().enumerate() {
            unsafe {
                ffi::Mat_VarSetCell(matcell_t, i as i32, mat.matvar_t);
            }
        }
        if matcell_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Mat::from_ptr(c_name.to_str().unwrap(), matcell_t)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::MatFile;

    use super::*;
    #[test]

    fn cell() {
        let c = Cell::new(1u32).push(1.23456f64).push("qwerty");
        let m: Mat = MayBeFrom::maybe_from("cell", c).unwrap();
        let matf = MatFile::save("cell.mat").unwrap();
        matf.write(m);
        // dbg!(m.len());
    }
}
