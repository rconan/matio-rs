use std::{ffi::CString, marker::PhantomData, ptr, rc::Rc};

use crate::{DataType, Mat, MatioError, Result};

pub trait MatTryInto<T> {
    fn maybe_into(self) -> Result<T>;
}
impl<'a, T: DataType> MatTryInto<T> for Mat<'a> {
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
impl<'a, T: DataType> MatTryInto<T> for &'a Mat<'a> {
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
impl<'a, T: DataType> MatTryInto<Vec<T>> for Mat<'a> {
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
impl<'a, T: DataType> MatTryInto<Vec<T>> for &'a Mat<'a> {
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
impl<'a> MatTryInto<Mat<'a>> for Mat<'a> {
    fn maybe_into(self) -> Result<Mat<'a>> {
        Ok(self)
    }
}

pub trait MatTryFrom<'a, T> {
    fn maybe_from<S: Into<String>>(name: S, data: T) -> Result<Self>
    where
        Self: Sized;
}
impl<'a, T: DataType + Copy> MatTryFrom<'a, T> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: T) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
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
impl<'a, T: DataType> MatTryFrom<'a, &'a Vec<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &'a Vec<T>) -> Result<Self> {
        <Mat<'a> as MatTryFrom<'a, &'a [T]>>::maybe_from(name, data.as_slice())
    }
}
impl<'a, T: DataType> MatTryFrom<'a, Vec<T>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: Vec<T>) -> Result<Self> {
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
            Mat::from_ptr(c_name.to_str().unwrap(), matvar_t)
        }
    }
}
impl<'a, T: DataType> MatTryFrom<'a, &'a [T]> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &'a [T]) -> Result<Self> {
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
            Mat::from_ptr(c_name.to_str().unwrap(), matvar_t)
        }
    }
}
impl<'a> MatTryFrom<'a, Vec<Mat<'a>>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, mut fields: Vec<Mat<'a>>) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let nfields = fields.len() as u32;
        let mut c_fields_ptr: Vec<_> = fields
            .iter()
            .map(|f| {
                CString::new(f.name.as_str())
                    .map(|f| f.into_raw() as *const i8)
                    .map_err(|e| MatioError::MatName(e))
            })
            .collect::<Result<Vec<*const i8>>>()?;
        let mut dims = [1u64, 1u64];
        let matvar_t = unsafe {
            ffi::Mat_VarCreateStruct(
                c_name.as_ptr(),
                2,
                dims.as_mut_ptr(),
                c_fields_ptr.as_mut_ptr(),
                nfields,
            )
        };
        if matvar_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            for field in fields.iter_mut() {
                let c_name = std::ffi::CString::new(field.name.as_str())?;
                let ptr = field.matvar_t as *mut ffi::matvar_t;
                unsafe {
                    ffi::Mat_VarSetStructFieldByName(matvar_t, c_name.as_ptr(), 0u64, ptr);
                }
            }
            Ok(Mat {
                name: c_name.to_str().unwrap().to_string(),
                matvar_t,
                fields: Some(fields),
                marker: PhantomData,
            })
        }
    }
}
type MatIterator<'a> = Vec<Box<dyn Iterator<Item = Mat<'a>>>>;
impl<'a> MatTryFrom<'a, MatIterator<'a>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, field_iter: MatIterator<'a>) -> Result<Self> {
        let fields: Vec<Vec<Mat<'a>>> = field_iter
            .into_iter()
            .map(|f| f.collect::<Vec<_>>())
            .collect();

        let c_name = std::ffi::CString::new(name.into())?;
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
            let c_name = std::ffi::CString::new(field_array[0].name.as_str())?;
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