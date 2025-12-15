
use crate::{
    DataType, Mat, MatioError, Result,
};
use std::ptr;

mod tuple;

/// Convert a [Mat] variable into a Rust data type
pub trait MayBeInto<T> {
    fn maybe_into(self) -> Result<T>;
}
macro_rules! maybe_into {
    ( $( $rs:ty ),+ ) => {
	    $(

            impl<'a> MayBeInto<$rs> for &Mat<'a> {
                fn maybe_into(self) -> Result<$rs> {
                    if self.len() > 1 {
                        return Err(MatioError::Scalar(self.name.clone(), self.len()));
                    }
                    match self.mat_type() {
                        Some(mat) if <$rs as DataType>::mat_type() == mat => {
                            Ok(unsafe { ((*self.matvar_t).data as *mut $rs).read() })
                        }
                        _ => Err(MatioError::TypeMismatch(
                            self.name.clone(),
                            <$rs as DataType>::to_string(),
                            self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
                        )),
                    }
                }
            }

        impl<'a> MayBeInto<$rs> for Mat<'a> {
            fn maybe_into(self) -> Result<$rs> {
                <&Mat<'a> as MayBeInto<$rs>>::maybe_into(&self)
            }
        }

        impl<'a> MayBeInto<Vec<$rs>> for &Mat<'a> {
            fn maybe_into(self) -> Result<Vec<$rs>> {
                match self.mat_type() {
                    Some(mat) if <$rs as DataType>::mat_type() == mat => {
                        let n = self.len();
                        let mut value: Vec<$rs> = Vec::with_capacity(n);
                        unsafe {
                            ptr::copy((*self.matvar_t).data as *mut $rs, value.as_mut_ptr(), n);
                            value.set_len(n);
                        }
                        Ok(value)
                    }
                    _ => Err(MatioError::TypeMismatch(
                        self.name.clone(),
                        <$rs as DataType>::to_string(),
                        self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
                    )),
                }
            }
        }


        impl<'a> MayBeInto<Vec<$rs>> for Mat<'a> {
            fn maybe_into(self) -> Result<Vec<$rs>> {
                <&Mat<'a> as MayBeInto<Vec<$rs>>>::maybe_into(&self)
            }
        }

        #[cfg(feature = "nalgebra")]
        impl<'a>
            MayBeInto<nalgebra::DMatrix<$rs>> for &Mat<'a>
        {
            fn maybe_into(self) -> Result<nalgebra::DMatrix<$rs>> {
                match self.mat_type() {
                    Some(mat) if <$rs as DataType>::mat_type() == mat => {
                        if self.rank() > 2 {
                            return Err(MatioError::Rank(self.rank()));
                        }
                        let dims = self.dims();
                        let (nrows, ncols) = (dims[0] as usize, dims[1] as usize);
                        let data: Vec<$rs> = self.maybe_into()?;
                        Ok(nalgebra::DMatrix::from_column_slice(
                            nrows,
                            ncols,
                            data.as_slice(),
                        ))
                    }
                    _ => Err(MatioError::TypeMismatch(
                        self.name.clone(),
                        <$rs as DataType>::to_string(),
                        self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
                    )),
                }
            }
        }

        #[cfg(feature = "nalgebra")]
        impl<'a>
        MayBeInto<nalgebra::DMatrix<$rs>> for Mat<'a>
        {
            fn maybe_into(self) -> Result<nalgebra::DMatrix<$rs>> {
                <&Mat<'a> as MayBeInto<nalgebra::DMatrix<$rs>>>::maybe_into(&self)
            }
        }

        #[cfg(feature = "faer")]
        impl<'a>
            MayBeInto<faer::mat::Mat<$rs>> for &Mat<'a>
        {
            fn maybe_into(self) -> Result<faer::mat::Mat<$rs>> {
                match self.mat_type() {
                    Some(mat) if <$rs as DataType>::mat_type() == mat => {
                        if self.rank() > 2 {
                            return Err(MatioError::Rank(self.rank()));
                        }
                        let dims = self.dims();
                        let (nrows, ncols) = (dims[0] as usize, dims[1] as usize);
                        let data: Vec<$rs> = self.maybe_into()?;
                        let mat = faer::MatRef::from_column_major_slice(data.as_slice(), nrows, ncols);
                        Ok(mat.cloned())
                    }
                    _ => Err(MatioError::TypeMismatch(
                        self.name.clone(),
                        <$rs as DataType>::to_string(),
                        self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
                    )),
                }
            }
        }

        #[cfg(feature = "faer")]
        impl<'a>
            MayBeInto<faer::mat::Mat<$rs>> for Mat<'a>
        {
            fn maybe_into(self) -> Result<faer::mat::Mat<$rs>> {
                <&Mat<'a> as MayBeInto<faer::mat::Mat<$rs>>>::maybe_into(&self)
            }
        }

        )+
    };
}

maybe_into! {
    f64,
    f32,
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64
}

impl<'a> MayBeInto<Mat<'a>> for Mat<'a> {
    fn maybe_into(self) -> Result<Mat<'a>> {
        Ok(self)
    }
}

impl<'a> MayBeInto<String> for Mat<'a> {
    fn maybe_into(self) -> Result<String> {
        match self.mat_type() {
            Some(mat) if <String as DataType>::mat_type() == mat => {
                let n = self.len();
                let mut value: Vec<u8> = Vec::with_capacity(n);
                unsafe {
                    ptr::copy((*self.matvar_t).data as *mut u8, value.as_mut_ptr(), n);
                    value.set_len(n);
                }
                Ok(String::from_utf8(value)?)
            }
            _ => Err(MatioError::TypeMismatch(
                self.name.clone(),
                <String as DataType>::to_string(),
                self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
            )),
        }
    }
}

impl<'a> MayBeInto<Vec<String>> for Mat<'a> {
    fn maybe_into(self) -> Result<Vec<String>> {
        match self.mat_type() {
            Some(mat) if <Vec<String> as DataType>::mat_type() == mat => {
                let n = self.len();
                let mut value: Vec<String> = Vec::with_capacity(n);
                for i in 0..n {
                    let matvar_t = unsafe { ffi::Mat_VarGetCell(self.matvar_t, i as i32) };
                    let mat = Mat::as_ptr(String::new(), matvar_t)?;
                    let rs = <Mat<'a> as MayBeInto<String>>::maybe_into(mat)?;
                    value.push(rs);
                }
                Ok(value)
            }
            _ => Err(MatioError::TypeMismatch(
                self.name.clone(),
                <Vec<String> as DataType>::to_string(),
                self.mat_type().map(|t| t.to_string()).unwrap_or_default(),
            )),
        }
    }
}

