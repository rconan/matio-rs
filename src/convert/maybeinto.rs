use std::ptr;

use crate::{DataType, Mat, MatioError, Result};

/// Convert a [Mat] variable into a Rust data type
pub trait MayBeInto<T> {
    fn maybe_into(self) -> Result<T>;
}
impl<'a, T: DataType> MayBeInto<T> for Mat<'a> {
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
impl<'a, T: DataType> MayBeInto<T> for &'a Mat<'a> {
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
impl<'a, T: DataType> MayBeInto<Vec<T>> for Mat<'a> {
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
impl<'a, T: DataType> MayBeInto<Vec<T>> for &'a Mat<'a> {
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
impl<'a> MayBeInto<Mat<'a>> for Mat<'a> {
    fn maybe_into(self) -> Result<Mat<'a>> {
        Ok(self)
    }
}
