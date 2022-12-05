use std::{ffi::CString, marker::PhantomData, ptr, vec};

use crate::{DataType, Mat, MatioError, Result};

/// Convert a Rust data type into a [Mat] variable
pub trait MayBeFrom<T> {
    fn maybe_from<S: Into<String>>(name: S, data: T) -> Result<Self>
    where
        Self: Sized;
}
impl<'a, T: DataType> MayBeFrom<T> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: T) -> Result<Self> {
        let c_name = CString::new(name.into())?;
        let mut dims = [1, 1];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                <T as DataType>::mat_c(),
                <T as DataType>::mat_t(),
                2,
                dims.as_mut_ptr(),
                &data as *const _ as *mut std::ffi::c_void,
                0,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Mat::from_ptr(c_name.to_str().unwrap(), matvar_t)
        }
    }
}
impl<'a, T: DataType> MayBeFrom<&Vec<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &Vec<T>) -> Result<Self> {
        <Mat<'a> as MayBeFrom<&[T]>>::maybe_from(name, data.as_slice())
    }
}
impl<'a, T: DataType> MayBeFrom<Vec<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: Vec<T>) -> Result<Self> {
        <Mat<'a> as MayBeFrom<&[T]>>::maybe_from(name, data.as_slice())
    }
}
impl<'a, T: DataType> MayBeFrom<&[T]> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &[T]) -> Result<Self> {
        let c_name = CString::new(name.into())?;
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
            Mat::from_ptr(c_name.to_str().unwrap(), matvar_t)
        }
    }
}
impl<'a> MayBeFrom<Vec<Mat<'a>>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, fields: Vec<Mat<'a>>) -> Result<Self> {
        let fields: VecArray = fields.into_iter().map(|mat| vec![mat]).collect();
        <Mat<'a> as MayBeFrom<Vec<Vec<Mat<'a>>>>>::maybe_from(name, fields)
    }
}
type VecArray<'a> = Vec<Vec<Mat<'a>>>;
impl<'a> MayBeFrom<VecArray<'a>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, fields: VecArray<'a>) -> Result<Self> {
        let c_name = CString::new(name.into())?;
        let mut dims = [1u64, fields[0].len() as u64];
        let matvar_t = unsafe {
            ffi::Mat_VarCreateStruct(c_name.as_ptr(), 2, dims.as_mut_ptr(), ptr::null_mut(), 0)
        };
        if matvar_t.is_null() {
            return Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ));
        }
        for field_array in &fields {
            let c_name = CString::new(field_array[0].name.as_str())?;
            unsafe {
                ffi::Mat_VarAddStructField(matvar_t, c_name.as_ptr());
            }
            for (index, field) in field_array.iter().enumerate() {
                let ptr = field.matvar_t as *mut ffi::matvar_t;
                unsafe {
                    ffi::Mat_VarSetStructFieldByName(matvar_t, c_name.as_ptr(), index as u64, ptr);
                }
            }
        }

        Ok(Mat {
            name: c_name.to_str().unwrap().to_string(),
            matvar_t,
            fields: Some(fields.into_iter().flatten().collect()),
            marker: PhantomData,
        })
    }
}

type MatIterator<'a> = Vec<Box<dyn Iterator<Item = Mat<'a>>>>;
impl<'a> MayBeFrom<MatIterator<'a>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, field_iter: MatIterator<'a>) -> Result<Self> {
        let fields: Vec<Vec<Mat<'a>>> = field_iter
            .into_iter()
            .map(|f| f.collect::<Vec<_>>())
            .collect();
        <Mat<'a> as MayBeFrom<Vec<Vec<Mat<'a>>>>>::maybe_from(name, fields)
    }
}

#[cfg(feature = "nalgebra")]
impl<'a, T: DataType> MayBeFrom<nalgebra::DVector<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, vector: nalgebra::DVector<T>) -> Result<Self>
    where
        Self: Sized,
    {
        <Mat<'a> as MayBeFrom<&nalgebra::DVector<T>>>::maybe_from(name, &vector)
    }
}
#[cfg(feature = "nalgebra")]
impl<'a, T: DataType> MayBeFrom<&nalgebra::DVector<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, vector: &nalgebra::DVector<T>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut dims: [u64; 2] = [vector.len() as u64, 1u64];
        let data = vector.as_slice();
        let c_name = CString::new(name.into())?;
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
            Mat::from_ptr(c_name.to_str().unwrap(), matvar_t)
        }
    }
}
#[cfg(feature = "nalgebra")]
impl<'a, T: DataType> MayBeFrom<nalgebra::DMatrix<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, matrix: nalgebra::DMatrix<T>) -> Result<Self>
    where
        Self: Sized,
    {
        <Mat<'a> as MayBeFrom<&nalgebra::DMatrix<T>>>::maybe_from(name, &matrix)
    }
}
#[cfg(feature = "nalgebra")]
impl<'a, T: DataType> MayBeFrom<&nalgebra::DMatrix<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, matrix: &nalgebra::DMatrix<T>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut dims: [u64; 2] = [matrix.nrows() as u64, matrix.ncols() as u64];
        let data = matrix.as_slice();
        let c_name = CString::new(name.into())?;
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
            Mat::from_ptr(c_name.to_str().unwrap(), matvar_t)
        }
    }
}
