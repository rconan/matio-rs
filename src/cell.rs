use std::{
    collections::VecDeque,
    ffi::CString,
    fmt::{Debug, Display},
};

use crate::{DataType, Mat, MatioError, MayBeFrom, MayBeInto, Result};

// https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15

pub struct Cell<T, C>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    C: CellBounds,
{
    item: T,
    next_cell: C,
}

pub struct LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    item: T,
}

// pub trait ItemBounds {} // where for <'a> Mat<'a>: MayBeFrom<T> {}
// impl<T> ItemBounds for T where for<'a> Mat<'a>: MayBeFrom<T> {}

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
}

impl<T: Debug> Debug for LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LastCell")
            .field("item", &self.item)
            .finish()
    }
}
impl<T: Display> Display for LastCell<T>
where
    for<'a> Mat<'a>: MayBeFrom<T> + MayBeInto<T>,
    LastCell<T>: CellBounds,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = <Self as CellBounds>::INDEX;
        writeln!(f, "{i}: {}", self.item)
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
}

impl<'a, T> TryFrom<LastCell<T>> for VecDeque<Mat<'a>>
where
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
{
    type Error = MatioError;

    fn try_from(last_cell: LastCell<T>) -> std::result::Result<Self, Self::Error> {
        let mat = <Mat<'a> as MayBeFrom<T>>::maybe_from(String::new(), last_cell.item);
        mat.map(|mat| vec![mat].into())
    }
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

impl<'a, T, C> MayBeFrom<Cell<T, C>> for Mat<'a>
where
    C: CellBounds,
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
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

impl<'a, T> MayBeInto<LastCell<T>> for Mat<'a>
where
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
    LastCell<T>: CellBounds,
{
    fn maybe_into(self) -> Result<LastCell<T>> {
        match self.mat_type() {
            Some(mat) if <Vec<String> as DataType>::mat_type() == mat => {
                let n = self.len();
                let i = n - <LastCell<T> as CellBounds>::INDEX - 1;
                let matvar_t = unsafe { ffi::Mat_VarGetCell(self.matvar_t, i as i32) };
                let mat = Mat::as_ptr(String::new(), matvar_t)?;
                let item = <Mat<'a> as MayBeInto<T>>::maybe_into(mat)?;
                Ok(LastCell { item })
            }
            _ => Err(MatioError::TypeMismatch(
                self.name.clone(),
                <Vec<String> as DataType>::to_string(),
                self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
            )),
        }
    }
}
impl<'a, T, C> MayBeInto<Cell<T, C>> for Mat<'a>
where
    C: CellBounds,
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
    for<'b> Mat<'b>: MayBeInto<C>,
{
    fn maybe_into(self) -> Result<Cell<T, C>> {
        match self.mat_type() {
            Some(mat) if <Vec<String> as DataType>::mat_type() == mat => {
                let n = self.len();
                let i = n - <Cell<T, C> as CellBounds>::INDEX - 1;
                let matvar_t = unsafe { ffi::Mat_VarGetCell(self.matvar_t, i as i32) };
                let mat = Mat::as_ptr(String::new(), matvar_t)?;
                let item = <Mat<'a> as MayBeInto<T>>::maybe_into(mat)?;
                let next_cell = <Mat<'a> as MayBeInto<C>>::maybe_into(self)?;
                Ok(Cell { item, next_cell })
            }
            _ => Err(MatioError::TypeMismatch(
                self.name.clone(),
                <Vec<String> as DataType>::to_string(),
                self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
            )),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::MatFile;

    use super::*;

    #[test]
    fn maybe_from() {
        let c = Cell::new(1u32).push(1.23456f64).push("qwerty".to_string());
        dbg!(&c);
        println!("{c}");
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
