use crate::{
    DataType, Mat, MatioError, MayBeInto, Result,
    cell::{Cell, CellBounds, LastCell},
};

impl<'a, T> MayBeInto<LastCell<T>> for Mat<'a>
where
    Mat<'a>: MayBeInto<T>,
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
    Mat<'a>: MayBeInto<T> + MayBeInto<C>,
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
