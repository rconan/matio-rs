use std::{ffi::CString, marker::PhantomData, ptr, vec};

use crate::{Mat, MatArray, MatioError, Result};

/// Convert a Rust data type into a [Mat] variable
pub trait MayBeFrom<T> {
    fn maybe_from<S: Into<String>>(name: S, data: T) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! maybe_from {
    ( $( ($rs:ty,$mat_c:expr,$mat_t:expr) ),+ ) => {
	    $(

            impl<'a> MayBeFrom<$rs> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, data: $rs) -> Result<Self> {
                    let c_name = CString::new(name.into())?;
                    let mut dims = [1, 1];
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
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

            impl<'a> MayBeFrom<&[$rs]> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, data: &[$rs]) -> Result<Self> {
                    let c_name = CString::new(name.into())?;
                    let mut dims = [1, data.len()];
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
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

            impl<'a> MayBeFrom<Vec<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, data: Vec<$rs>) -> Result<Self> {
                    MayBeFrom::<&[$rs]>::maybe_from(name, data.as_slice())
                }
            }

            impl<'a> MayBeFrom<&Vec<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, data: &Vec<$rs>) -> Result<Self> {
                    MayBeFrom::<&[$rs]>::maybe_from(name, data.as_slice())
                }
            }

            impl<'a> MayBeFrom<MatArray<'a, $rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, mat_array: MatArray<'a, $rs>) -> Result<Self> {
                    let c_name = CString::new(name.into())?;
                    // let mut dims = [1, data.len() as u64];
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
                            mat_array.dims.len() as i32,
                            mat_array.dims.as_ptr() as *mut _,
                            mat_array.data.as_ptr() as *mut std::ffi::c_void,
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
            impl<'a> MayBeFrom<nalgebra::DVector<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, vector: nalgebra::DVector<$rs>) -> Result<Self>
                where
                    Self: Sized,
                {
                    <Mat<'a> as MayBeFrom<&nalgebra::DVector<$rs>>>::maybe_from(name, &vector)
                }
            }
            #[cfg(feature = "nalgebra")]
            impl<'a> MayBeFrom<&nalgebra::DVector<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, vector: &nalgebra::DVector<$rs>) -> Result<Self>
                where
                    Self: Sized,
                {
                    let mut dims: [usize; 2] = [vector.len(), 1usize];
                    let data = vector.as_slice();
                    let c_name = CString::new(name.into())?;
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
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
            impl<'a> MayBeFrom<nalgebra::DMatrix<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, matrix: nalgebra::DMatrix<$rs>) -> Result<Self>
                where
                    Self: Sized,
                {
                    <Mat<'a> as MayBeFrom<&nalgebra::DMatrix<$rs>>>::maybe_from(name, &matrix)
                }
            }
            #[cfg(feature = "nalgebra")]
            impl<'a> MayBeFrom<&nalgebra::DMatrix<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, matrix: &nalgebra::DMatrix<$rs>) -> Result<Self>
                where
                    Self: Sized,
                {
                    let mut dims: [usize; 2] = [matrix.nrows(), matrix.ncols()];
                    let data = matrix.as_slice();
                    let c_name = CString::new(name.into())?;
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
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

            #[cfg(feature = "faer")]
            impl<'a> MayBeFrom<faer::mat::MatRef<'a,$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, matrix: faer::mat::MatRef<'a,$rs>) -> Result<Self>
                where
                    Self: Sized,
                {
                    let mut dims: [usize; 2] = [matrix.nrows(), matrix.ncols()];
                    let data: Vec<_> = matrix
                        .col_iter()
                        .map(|c| c.iter().cloned().collect::<Vec<$rs>>())
                        .collect();
                    let c_name = CString::new(name.into())?;
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
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
            #[cfg(feature = "faer")]
            impl<'a> MayBeFrom<&faer::mat::Mat<$rs>> for Mat<'a> {
                fn maybe_from<S: Into<String>>(name: S, matrix: &faer::mat::Mat<$rs>) -> Result<Self>
                where
                    Self: Sized,
                {
                    let mut dims: [usize; 2] = [matrix.nrows(), matrix.ncols()];
                    let data: Vec<_> = matrix
                        .col_iter()
                        .flat_map(|c| c.iter().cloned().collect::<Vec<$rs>>())
                        .collect();
                    let c_name = CString::new(name.into())?;
                    let matvar_t = unsafe {
                        ffi::Mat_VarCreate(
                            c_name.as_ptr(),
                            $mat_c,
                            $mat_t,
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

		)+
    };
}

maybe_from! {
    (f64,ffi::matio_classes_MAT_C_DOUBLE,ffi::matio_types_MAT_T_DOUBLE),
    (f32,ffi::matio_classes_MAT_C_SINGLE,ffi::matio_types_MAT_T_SINGLE),
    (i8,ffi::matio_classes_MAT_C_INT8,ffi::matio_types_MAT_T_INT8),
    (i16,ffi::matio_classes_MAT_C_INT16,ffi::matio_types_MAT_T_INT16),
    (i32,ffi::matio_classes_MAT_C_INT32,ffi::matio_types_MAT_T_INT32),
    (i64,ffi::matio_classes_MAT_C_INT64,ffi::matio_types_MAT_T_INT64),
    (u8,ffi::matio_classes_MAT_C_UINT8,ffi::matio_types_MAT_T_UINT8),
    (u16,ffi::matio_classes_MAT_C_UINT16,ffi::matio_types_MAT_T_UINT16),
    (u32,ffi::matio_classes_MAT_C_UINT32,ffi::matio_types_MAT_T_UINT32),
    (u64,ffi::matio_classes_MAT_C_UINT64,ffi::matio_types_MAT_T_UINT64)
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
        let mut dims = [1usize, fields[0].len()];
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
                    ffi::Mat_VarSetStructFieldByName(matvar_t, c_name.as_ptr(), index, ptr);
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
