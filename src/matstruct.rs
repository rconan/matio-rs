use crate::{matvar::DataType, MatObjects, MatVar, MatioError, Result};
use std::ffi::CString;

/// Matlab structure
pub struct MatStruct {
    matstruct_t: *mut ffi::matvar_t,
    objects: Vec<Box<dyn MatObjects>>,
}

impl MatStruct {
    /// Creates a new Matlab structure `name`
    pub fn new<S: Into<String>>(name: S, fields: Vec<S>) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let nfields = fields.len() as u32;
        let mut c_fields_ptr: Vec<_> = fields
            .into_iter()
            .map(|f| {
                CString::new(f.into())
                    .map(|f| f.into_raw() as *const i8)
                    .map_err(|e| MatioError::MatName(e))
            })
            .collect::<Result<Vec<*const i8>>>()?;
        let mut dims = [1, 1];
        let matstruct_t = unsafe {
            ffi::Mat_VarCreateStruct(
                c_name.as_ptr(),
                2,
                dims.as_mut_ptr(),
                c_fields_ptr.as_mut_ptr(),
                nfields,
            )
        };
        if matstruct_t.is_null() {
            Err(MatioError::MatVarCreate(
                c_name.to_str().unwrap().to_string(),
            ))
        } else {
            Ok(Self {
                matstruct_t,
                objects: Vec::new(),
            })
        }
    }
}

pub trait Field<S: Into<String>, T> {
    /// Adds a Matlab variable to the field
    fn field(self, name: S, data: &T) -> Result<Self>
    where
        Self: Sized;
}
impl<S, T> Field<S, T> for MatStruct
where
    S: Into<String>,
    T: 'static + DataType+Copy,
{
    fn field(mut self, name: S, data: &T) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut fieldvar = MatVar::<T>::new(String::new(), *data)?;
        let ptr = fieldvar.as_mut_ptr();
        self.objects.push(Box::new(fieldvar));
        unsafe { ffi::Mat_VarSetStructFieldByName(self.matstruct_t, c_name.as_ptr(), 0, ptr) };
        Ok(self)
    }
}
impl<S, T> Field<S, Vec<T>> for MatStruct
where
    S: Into<String>,
    T: 'static + DataType,
{
    fn field(mut self, name: S, mut data: &Vec<T>) -> Result<Self> {
        let c_name = std::ffi::CString::new(name.into())?;
        let mut fieldvar = MatVar::<Vec<T>>::new(String::new(), data)?;
        let ptr = fieldvar.as_mut_ptr();
        self.objects.push(Box::new(fieldvar));
        unsafe { ffi::Mat_VarSetStructFieldByName(self.matstruct_t, c_name.as_ptr(), 0, ptr) };
        Ok(self)
    }
}
impl MatObjects for MatStruct {
    fn as_mut_ptr(&mut self) -> *mut ffi::matvar_t {
        self.matstruct_t
    }
}
