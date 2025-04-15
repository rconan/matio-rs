use crate::{DataType, Mat, MatioError, Result};
use std::ptr;

/// Convert a [Mat] variable into a Rust data type
pub trait MayBeInto<T> {
    fn maybe_into(self) -> Result<T>;
}
impl<'a, T: DataType> MayBeInto<T> for &Mat<'a> {
    fn maybe_into(self) -> Result<T> {
        if self.len() > 1 {
            return Err(MatioError::Scalar(self.name.clone(), self.len()));
        }
        if T::mat_type() == self.mat_type() {
            Ok(unsafe { ((*self.matvar_t).data as *mut T).read() })
        } else {
            Err(MatioError::TypeMismatch(
                self.name.clone(),
                T::to_string(),
                self.mat_type().to_string(),
            ))
        }
    }
}
impl<'a, T: DataType> MayBeInto<T> for Mat<'a> {
    fn maybe_into(self) -> Result<T> {
        <&Mat<'a> as MayBeInto<T>>::maybe_into(&self)
    }
}

impl<'a, T: DataType> MayBeInto<Vec<T>> for &Mat<'a> {
    fn maybe_into(self) -> Result<Vec<T>> {
        if T::mat_type() != self.mat_type() {
            return Err(MatioError::TypeMismatch(
                self.name.clone(),
                T::to_string(),
                self.mat_type().to_string(),
            ));
        }
        let n = self.len();
        let mut value: Vec<T> = Vec::with_capacity(n);
        unsafe {
            ptr::copy((*self.matvar_t).data as *mut T, value.as_mut_ptr(), n);
            value.set_len(n);
        }
        Ok(value)
    }
}
impl<'a, T: DataType> MayBeInto<Vec<T>> for Mat<'a> {
    fn maybe_into(self) -> Result<Vec<T>> {
        <&Mat<'a> as MayBeInto<Vec<T>>>::maybe_into(&self)
    }
}
impl<'a> MayBeInto<Mat<'a>> for Mat<'a> {
    fn maybe_into(self) -> Result<Mat<'a>> {
        Ok(self)
    }
}

#[cfg(feature = "nalgebra")]
impl<'a, T: DataType + Clone + std::cmp::PartialEq + std::fmt::Debug + 'static>
    MayBeInto<nalgebra::DMatrix<T>> for Mat<'a>
{
    fn maybe_into(self) -> Result<nalgebra::DMatrix<T>> {
        <&Mat<'a> as MayBeInto<nalgebra::DMatrix<T>>>::maybe_into(&self)
    }
}
#[cfg(feature = "nalgebra")]
impl<'a, T: DataType + Clone + std::cmp::PartialEq + std::fmt::Debug + 'static>
    MayBeInto<nalgebra::DMatrix<T>> for &Mat<'a>
{
    fn maybe_into(self) -> Result<nalgebra::DMatrix<T>> {
        if T::mat_type() != self.mat_type() {
            return Err(MatioError::TypeMismatch(
                self.name.clone(),
                T::to_string(),
                self.mat_type().to_string(),
            ));
        }
        if self.rank() > 2 {
            return Err(MatioError::Rank(self.rank()));
        }
        let dims = self.dims();
        let (nrows, ncols) = (dims[0] as usize, dims[1] as usize);
        let data: Vec<T> = self.maybe_into()?;
        Ok(nalgebra::DMatrix::from_column_slice(
            nrows,
            ncols,
            data.as_slice(),
        ))
    }
}

#[cfg(feature = "faer")]
impl<'a, T: DataType + Clone + std::cmp::PartialEq + std::fmt::Debug + 'static>
    MayBeInto<faer::mat::Mat<T>> for Mat<'a>
{
    fn maybe_into(self) -> Result<faer::mat::Mat<T>> {
        <&Mat<'a> as MayBeInto<faer::mat::Mat<T>>>::maybe_into(&self)
    }
}
#[cfg(feature = "faer")]
impl<'a, T: DataType + Clone + std::cmp::PartialEq + std::fmt::Debug + 'static>
    MayBeInto<faer::mat::Mat<T>> for &Mat<'a>
{
    fn maybe_into(self) -> Result<faer::mat::Mat<T>> {
        if T::mat_type() != self.mat_type() {
            return Err(MatioError::TypeMismatch(
                self.name.clone(),
                T::to_string(),
                self.mat_type().to_string(),
            ));
        }
        if self.rank() > 2 {
            return Err(MatioError::Rank(self.rank()));
        }
        let dims = self.dims();
        let (nrows, ncols) = (dims[0] as usize, dims[1] as usize);
        let data: Vec<T> = self.maybe_into()?;
        let mat = faer::MatRef::from_column_major_slice(data.as_slice(), nrows, ncols);
        Ok(mat.cloned())
    }
}
