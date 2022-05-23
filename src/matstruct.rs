use crate::{matvar::DataType, MatObject, MatVar, MatioError, Result};
use std::{collections::HashMap, ffi::CString};

/// Matlab structure
pub struct MatStruct {
    matstruct_t: *mut ffi::matvar_t,
    #[allow(dead_code)]
    fields: Option<HashMap<String, Vec<Box<dyn MatObject>>>>,
}
/// Matlab structure builder
pub struct MatStructBuilder {
    name: String,
    fields: Option<HashMap<String, Vec<Box<dyn MatObject>>>>,
}
impl MatStruct {
    /// Creates a new Matlab structure `name`
    pub fn new<S: Into<String>>(name: S) -> MatStructBuilder {
        MatStructBuilder {
            name: name.into(),
            fields: None,
        }
    }
}
impl MatStructBuilder {
    /// Build a Matlab structure
    pub fn build(self) -> Result<MatStruct> {
        let c_name = std::ffi::CString::new(self.name)?;
        match self.fields {
            Some(mut fields) => {
                let nfields = fields.len() as u32;
                let mut c_fields_ptr: Vec<_> = fields
                    .keys()
                    .map(|f| {
                        CString::new(f.as_str())
                            .map(|f| f.into_raw() as *const i8)
                            .map_err(|e| MatioError::MatName(e))
                    })
                    .collect::<Result<Vec<*const i8>>>()?;
                let mut n: Vec<_> = fields.values().map(|v| v.len()).collect();
                n.dedup();
                let mut dims = match n.len() {
                    l if l == 0 || l > 1 => return Err(MatioError::FieldSize(n)),
                    _ => [1u64, n[0] as u64],
                };
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
                    for (key, val) in fields.iter_mut() {
                        let c_name = std::ffi::CString::new(key.as_str())?;
                        for (index, fieldvar) in val.iter_mut().enumerate() {
                            let ptr = fieldvar.as_mut_ptr();
                            unsafe {
                                ffi::Mat_VarSetStructFieldByName(
                                    matstruct_t,
                                    c_name.as_ptr(),
                                    index as u64,
                                    ptr,
                                );
                            }
                        }
                    }
                    Ok(MatStruct {
                        matstruct_t,
                        fields: Some(fields),
                    })
                }
            }
            None => Err(MatioError::NoFields),
        }
    }
}

/*impl Drop for MatStruct {
    fn drop(&mut self) {
        unsafe {
            ffi::Mat_VarFree(self.matstruct_t);
        }
    }
}*/

impl MatObject for MatStruct {
    fn as_mut_ptr(&mut self) -> *mut ffi::matvar_t {
        self.matstruct_t
    }
}

/// Matlab field structure interface
pub trait Field<T> {
    /// Adds a Matlab variable to the field `name`
    fn field<S: Into<String>>(self, name: S, data: &T) -> Result<Self>
    where
        Self: Sized;
}
impl<T> Field<T> for MatStructBuilder
where
    T: 'static + DataType + Copy,
{
    fn field<S: Into<String>>(mut self, name: S, data: &T) -> Result<Self> {
        let fieldvar = MatVar::<T>::new(String::new(), *data)?;
        self.fields
            .get_or_insert_with(|| HashMap::new())
            .entry(name.into())
            .or_default()
            .push(Box::new(fieldvar));
        Ok(self)
    }
}
impl<T> Field<Vec<T>> for MatStructBuilder
where
    T: 'static + DataType,
{
    fn field<S: Into<String>>(mut self, name: S, data: &Vec<T>) -> Result<Self> {
        let fieldvar = MatVar::<Vec<T>>::new(String::new(), data)?;
        self.fields
            .get_or_insert_with(|| HashMap::new())
            .entry(name.into())
            .or_default()
            .push(Box::new(fieldvar));
        if self.fields.is_none() {
            self.fields = Some(HashMap::new());
        }
        Ok(self)
    }
}

/// Matlab field structure interface for [Iterator]
pub trait FieldIterator<'a, T> {
    /// Adds a Matlab variable to the field `name`
    fn field<S: Into<String> + Clone>(
        self,
        name: S,
        data: impl Iterator<Item = &'a T>,
    ) -> Result<Self>
    where
        T: 'a,
        Self: Sized;
}
impl<'a, T> FieldIterator<'a, T> for MatStructBuilder
where
    T: 'static + DataType + Copy,
{
    fn field<S>(mut self, name: S, data: impl Iterator<Item = &'a T>) -> Result<Self>
    where
        S: Into<String> + Clone,
    {
        self.fields
            .get_or_insert_with(|| HashMap::new())
            .entry(name.clone().into())
            .or_default()
            .extend(data.map(|data| {
                Box::new(
                    MatVar::<T>::new(String::new(), *data)
                        .expect(&format!("creating mat var {0} failed", name.clone().into())),
                ) as Box<dyn MatObject>
            }));
        Ok(self)
    }
}
impl<'a, T> FieldIterator<'a, Vec<T>> for MatStructBuilder
where
    T: 'static + DataType,
{
    fn field<S>(mut self, name: S, data: impl Iterator<Item = &'a Vec<T>>) -> Result<Self>
    where
        S: Into<String> + Clone,
    {
        self.fields
            .get_or_insert_with(|| HashMap::new())
            .entry(name.clone().into())
            .or_default()
            .extend(data.map(|data| {
                Box::new(
                    MatVar::<Vec<T>>::new(String::new(), data)
                        .expect(&format!("creating mat var {0} failed", name.clone().into())),
                ) as Box<dyn MatObject>
            }));
        Ok(self)
    }
}

/// Matlab field structure interface for [MatObject]
pub trait FieldMatObject<T: MatObject> {
    fn field<S: Into<String>>(self, name: S, data: T) -> Result<Self>
    where
        Self: Sized;
}
impl<T> FieldMatObject<T> for MatStructBuilder
where
    T: 'static + MatObject,
{
    fn field<S: Into<String>>(mut self, name: S, data: T) -> Result<Self> {
        self.fields
            .get_or_insert_with(|| HashMap::new())
            .entry(name.into())
            .or_default()
            .push(Box::new(data));
        Ok(self)
    }
}

/// Matlab field structure interface for [MatObject] [Iterator]
pub trait FieldMatObjectIterator<T: MatObject> {
    fn field<S: Into<String>>(self, name: S, data: impl Iterator<Item = T>) -> Result<Self>
    where
        Self: Sized;
}
impl<T> FieldMatObjectIterator<T> for MatStructBuilder
where
    T: 'static + MatObject,
{
    fn field<S: Into<String>>(mut self, name: S, data: impl Iterator<Item = T>) -> Result<Self> {
        self.fields
            .get_or_insert_with(|| HashMap::new())
            .entry(name.into())
            .or_default()
            .extend(data.map(|data| Box::new(data) as Box<dyn MatObject>));
        Ok(self)
    }
}
