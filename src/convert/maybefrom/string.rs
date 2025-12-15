use std::ffi::CString;

use crate::{Mat, MatioError, Result};

use super::MayBeFrom;

impl<'a> MayBeFrom<&str> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &str) -> Result<Self> {
        let c_name = CString::new(name.into())?;
        let mut dims = [1, data.len()];
        let matvar_t = unsafe {
            ffi::Mat_VarCreate(
                c_name.as_ptr(),
                ffi::matio_classes_MAT_C_CHAR,
                ffi::matio_types_MAT_T_UINT8,
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

impl<'a> MayBeFrom<String> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: String) -> Result<Self> {
        <Mat<'a> as MayBeFrom<&str>>::maybe_from(name, data.as_str())
    }
}

impl<'a> MayBeFrom<&String> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &String) -> Result<Self> {
        <Mat<'a> as MayBeFrom<&str>>::maybe_from(name, data.as_str())
    }
}

impl<'a> MayBeFrom<&[&str]> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &[&str]) -> Result<Self> {
        let c_name = CString::new(name.into())?;
        let mut dims = [1, data.len()];
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
        for (i, s) in data.into_iter().enumerate() {
            let mut dims = [1, s.len()];
            unsafe {
                let matvar_t = ffi::Mat_VarCreate(
                    std::ptr::null_mut(),
                    ffi::matio_classes_MAT_C_CHAR,
                    ffi::matio_types_MAT_T_UINT8,
                    2,
                    dims.as_mut_ptr(),
                    s.as_ptr() as *mut std::ffi::c_void,
                    0,
                );
                ffi::Mat_VarSetCell(matcell_t, i as i32, matvar_t);
            };
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
impl<'a> MayBeFrom<Vec<&str>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: Vec<&str>) -> Result<Self>
    where
        Self: Sized,
    {
        <Mat<'a> as MayBeFrom<&[&str]>>::maybe_from(name, data.as_slice())
    }
}
impl<'a> MayBeFrom<Vec<String>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: Vec<String>) -> Result<Self>
    where
        Self: Sized,
    {
        let data: Vec<_> = data.iter().map(|s| s.as_str()).collect();
        <Mat<'a> as MayBeFrom<&[&str]>>::maybe_from(name, data.as_slice())
    }
}
impl<'a> MayBeFrom<Vec<&String>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: Vec<&String>) -> Result<Self>
    where
        Self: Sized,
    {
        let data: Vec<_> = data.iter().map(|s| s.as_str()).collect();
        <Mat<'a> as MayBeFrom<Vec<&str>>>::maybe_from(name, data)
    }
}
impl<'a> MayBeFrom<&Vec<&str>> for Mat<'a> {
    fn maybe_from<S: Into<String>>(name: S, data: &Vec<&str>) -> Result<Self>
    where
        Self: Sized,
    {
        <Mat<'a> as MayBeFrom<&[&str]>>::maybe_from(name, data)
    }
}
