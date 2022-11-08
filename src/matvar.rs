use crate::{MatObject, MatioError, Result};
use std::{
    any::{type_name, TypeId},
    fmt::Display,
    marker::PhantomData,
    ptr,
};

mod datatype;
pub use datatype::DataType;

/// Matlab variable
pub struct MatVar<T> {
    pub(crate) matvar_t: *mut ffi::matvar_t,
    pub(crate) data_type: PhantomData<T>,
}
impl<T> Drop for MatVar<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::Mat_VarFree(self.matvar_t);
        }
    }
}
impl<T> Display for MatVar<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { ffi::Mat_VarPrint(self.matvar_t, 0) }
        Ok(())
    }
}
impl<T: 'static> MatVar<T> {
    /// Checks Rust and Matlab types compatibility
    pub fn match_types(self) -> Result<Self> {
        unsafe {
            if (TypeId::of::<T>() == TypeId::of::<f64>()
                || TypeId::of::<T>() == TypeId::of::<Vec<f64>>())
                && (*self.matvar_t).data_type == ffi::matio_types_MAT_T_DOUBLE
            {
                return Ok(self);
            }
            Err(MatioError::MatType(
                type_name::<T>().to_string(),
                {
                    match (*self.matvar_t).data_type {
                        ffi::matio_types_MAT_T_DOUBLE => "DOUBLE",
                        _ => "UNKNOWN",
                    }
                }
                .to_string(),
            ))
        }
    }
}

impl<T> MatObject for MatVar<T> {
    fn as_mut_ptr(&mut self) -> *mut ffi::matvar_t {
        self.matvar_t
    }
    fn as_ptr(&self) -> *const ffi::matvar_t {
        self.matvar_t
    }
}
impl<T: DataType> MatVar<T> {
    /// Creates a new Matlab variable `name`
    pub fn new<S: Into<String>>(name: S, data: &T) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut dims = [1, 1];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                <T as DataType>::mat_c(),
                <T as DataType>::mat_t(),
                2,
                dims.as_mut_ptr(),
                data as *const _ as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatVar {
                matvar_t,
                data_type: PhantomData,
            })
        }
    }
}
impl<T: DataType> MatVar<Vec<T>> {
    /// Creates a new Matlab variable `name`
    pub fn new<S: Into<String>>(name: S, data: &[T]) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut dims = [1, data.len() as u64];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                <T as DataType>::mat_c(),
                <T as DataType>::mat_t(),
                2,
                dims.as_mut_ptr(),
                data.as_ptr() as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatVar {
                matvar_t,
                data_type: PhantomData,
            })
        }
    }
    /// Creates a new Matlab 2D variable `name` column-wise
    pub fn array<S: Into<String>>(name: S, data: &[T], shape: (usize, usize)) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut dims = [shape.0 as u64, shape.1 as u64];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                <T as DataType>::mat_c(),
                <T as DataType>::mat_t(),
                2,
                dims.as_mut_ptr(),
                data.as_ptr() as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(MatVar {
                matvar_t,
                data_type: PhantomData,
            })
        }
    }
}

macro_rules! scalar {
    ( $( $rs:ty ),+ ) => {
	$(
        impl From<MatVar<$rs>> for $rs {
            fn from(mat_var: MatVar<$rs>) -> Self {
                unsafe { ((*mat_var.matvar_t).data as *mut $rs).read() }
            }
        }
        impl<S:Into<String>+ Clone> From<(S,$rs)> for MatVar<$rs> {
            fn from((name,data): (S, $rs)) -> Self {
                MatVar::<$rs>::new(name.clone(),&data).expect(&format!("failed to create Matlab variable {:}",name.into()))
            }
        }
        impl<S:Into<String>+ Clone> From<(S,&$rs)> for MatVar<$rs> {
            fn from((name,data): (S, &$rs)) -> Self {
                MatVar::<$rs>::new(name.clone(),data).expect(&format!("failed to create Matlab variable {:}",name.into()))
            }
        }
    )+
    };
}
scalar!(f64, f32, i8, i16, i32, i64, u8, u16, u32, u64);

impl<T: DataType> From<MatVar<Vec<T>>> for Vec<T> {
    fn from(mat_var: MatVar<Vec<T>>) -> Self {
        unsafe {
            let rank = (*mat_var.matvar_t).rank as usize;
            let mut dims: Vec<u64> = Vec::with_capacity(rank);
            ptr::copy((*mat_var.matvar_t).dims, dims.as_mut_ptr(), rank);
            dims.set_len(rank);
            let length = dims.into_iter().product::<u64>() as usize;

            let mut value: Vec<T> = Vec::with_capacity(length);
            ptr::copy(
                (*mat_var.matvar_t).data as *mut T,
                value.as_mut_ptr(),
                length,
            );
            value.set_len(length);
            value
        }
    }
}
impl<S: Into<String> + Clone, T: DataType> From<(S, &Vec<T>)> for MatVar<Vec<T>> {
    fn from((name, data): (S, &Vec<T>)) -> Self {
        MatVar::<Vec<T>>::new(name.clone(), data).expect(&format!(
            "failed to create Matlab variable {:}",
            name.into()
        ))
    }
}

#[cfg(feature = "nalgebra")]
impl<T> From<MatVar<Vec<T>>> for nalgebra::DVector<T>
where
    T: 'static + DataType + Clone + std::fmt::Debug + PartialEq,
{
    fn from(mat_var: MatVar<Vec<T>>) -> Self {
        let v: Vec<T> = mat_var.into();
        nalgebra::DVector::from_vec(v)
    }
}
#[cfg(feature = "nalgebra")]
impl<T> From<MatVar<Vec<T>>> for Option<nalgebra::DMatrix<T>>
where
    T: 'static + DataType + Clone + std::fmt::Debug + PartialEq,
{
    fn from(mat_var: MatVar<Vec<T>>) -> Self {
        unsafe {
            let rank = (*mat_var.matvar_t).rank as usize;
            if rank > 2 {
                None
            } else {
                let mut dims: Vec<u64> = Vec::with_capacity(rank);
                ptr::copy((*mat_var.matvar_t).dims, dims.as_mut_ptr(), rank);
                dims.set_len(rank);
                let (n, m) = (dims[0] as usize, dims[1] as usize);
                let length = dims.into_iter().product::<u64>() as usize;
                let mut value: Vec<T> = Vec::with_capacity(length);
                ptr::copy(
                    (*mat_var.matvar_t).data as *mut T,
                    value.as_mut_ptr(),
                    length,
                );
                value.set_len(length);
                Some(nalgebra::DMatrix::from_vec(n, m, value))
            }
        }
    }
}
