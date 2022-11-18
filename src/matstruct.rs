use crate::{DataType, Mat, MatioError, Result};
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::Display,
    marker::PhantomData,
};

/// Matlab structure
pub struct MatStruct<'a> {
    pub(crate) matvar_t: *mut ffi::matvar_t,
    #[allow(dead_code)]
    pub fields: Option<Vec<Mat<'a>>>,
}
/// Matlab structure builder
pub struct MatStructBuilder<'a> {
    name: String,
    fields: Option<Vec<Mat<'a>>>,
}
impl<'a> MatStruct<'a> {
    /// Creates a new Matlab structure `name`
    pub fn new<S: Into<String>>(name: S) -> MatStructBuilder<'a> {
        MatStructBuilder {
            name: name.into(),
            fields: None,
        }
    }
    /*     /// Get the number of fields
    pub fn n_field(&self) -> usize {
        unsafe { ffi::Mat_VarGetNumberOfFields(self.matvar_t) as usize }
    }
    /// Get the field names
    pub fn fields_name(&self) -> std::result::Result<Vec<&str>, std::str::Utf8Error> {
        unsafe {
            let c_str: Vec<*mut i8> = {
                let n = self.n_field();
                Vec::from_raw_parts(
                    ffi::Mat_VarGetStructFieldnames(self.matvar_t) as *mut *mut i8,
                    n,
                    n,
                )
            };
            c_str
                .into_iter()
                .map(|s| CStr::from_ptr(s).to_str())
                .collect()
        }
    }
    /// Get the name of the structure
    pub fn name(&self) -> std::result::Result<String, std::str::Utf8Error> {
        let c_str = unsafe {
            let ptr = (*self.matvar_t).name;
            CStr::from_ptr(ptr as *const i8)
        };
        c_str.to_str().map(|s| s.into())
    } */
}
/* impl Display for MatStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"Matlab struct "{}" of dims:{:?}"#,
            self.name().map_err(|_| std::fmt::Error)?,
            self.dims()
        )
        /*         writeln!(
            f,
            " with fields: {:?}",
            self.fields_name().map_err(|_| std::fmt::Error)?
        ) */
    }
} */
impl<'a> MatStructBuilder<'a> {
    /// Build a Matlab structure
    pub fn build(self) -> Result<Mat<'a>> {
        let c_name = std::ffi::CString::new(self.name)?;
        match self.fields {
            Some(mut fields) => {
                let nfields = fields.len() as u32;
                dbg!(nfields);
                let mut c_fields_ptr: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        CString::new(f.name.as_str())
                            .map(|f| f.into_raw() as *const i8)
                            .map_err(|e| MatioError::MatName(e))
                    })
                    .collect::<Result<Vec<*const i8>>>()?;
                let mut n: Vec<_> = fields
                    .iter()
                    .filter_map(|v| v.fields.as_ref().map(|x| x.len()))
                    .collect();
                n.dedup();
                /*                 let mut dims = match n.len() {
                    l if l == 0 || l > 1 => return Err(MatioError::FieldSize(n)),
                    _ => [1u64, n[0] as u64],
                }; */
                let mut dims = [1u64, 1u64];
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
                    /*                     for (index, field) in fields.iter_mut().enumerate() {
                        let c_name = std::ffi::CString::new(field.name.as_str())?;
                        // for (index, fieldvar) in
                        //     field.fields.as_mut().unwrap().iter_mut().enumerate()
                        // {
                        dbg!(index);
                        let ptr = field.matvar_t as *mut ffi::matvar_t;
                        unsafe {
                            ffi::Mat_VarSetStructFieldByName(
                                matstruct_t,
                                c_name.as_ptr(),
                                index as u64,
                                ptr,
                            );
                            // }
                        }
                    } */
                    let field = fields.get(0).unwrap();
                    let c_name = std::ffi::CString::new(field.name.as_str())?;
                    dbg!(&c_name);
                    let ptr = field.matvar_t as *mut ffi::matvar_t;
                    unsafe {
                        ffi::Mat_VarSetStructFieldByName(matstruct_t, c_name.as_ptr(), 0u64, ptr);
                        // }
                    }
                    /* let field = fields.get(1).unwrap();
                    let c_name = std::ffi::CString::new(field.name.as_str())?;
                    dbg!(&c_name);
                    let ptr = field.matvar_t as *mut ffi::matvar_t;
                    unsafe {
                        ffi::Mat_VarSetStructFieldByName(matstruct_t, c_name.as_ptr(), 1u64, ptr);
                        // }
                    } */
                    Ok(Mat {
                        name: c_name.to_str().unwrap().to_string(),
                        matvar_t: matstruct_t,
                        fields: Some(fields),
                        marker: PhantomData,
                    })
                }
            }
            None => Err(MatioError::NoFields),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{MatFile, MatTryFrom, MatTryInto};

    use super::*;
    #[test]
    fn test_create() {
        let mat_a = Mat::maybe_from("fa", 123f64).unwrap();
        let v = vec![0i32, 1, 2, 3, 4];
        // let mat_v = Mat::maybe_from("fb", &v).unwrap();

        let mut matb = MatStruct::new("s");
        matb.fields = Some(vec![mat_a]);
        let mat = matb.build().unwrap();
        dbg!("HERE");
        // let fields = mat.fields.as_ref.unwrap();
        // dbg!(fields.len());
        /*         if let Some(mat) = mat.fields.as_ref().unwrap().get(0) {
            let fa: f64 = mat.maybe_into().unwrap();
            dbg!(fa);
        };
        if let Some(mat) = mat.fields.as_ref().unwrap().get(1) {
            let fa: Vec<i32> = mat.maybe_into().unwrap();
            dbg!(fa);
        }; */

        // let mat_file = MatFile::save(Path::new("data").join("struct.mat")).unwrap();
        // mat_file.write(mat);
    }
}

/*impl Drop for MatStruct {
    fn drop(&mut self) {
        unsafe {
            ffi::Mat_VarFree(self.matstruct_t);
        }
    }
}*/

/* impl MatObject for MatStruct {
    fn as_mut_ptr(&mut self) -> *mut ffi::matvar_t {
        self.matvar_t
    }
    fn as_ptr(&self) -> *const ffi::matvar_t {
        self.matvar_t
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
        let fieldvar = MatVar::<T>::new(String::new(), data)?;
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
                    MatVar::<T>::new(String::new(), data)
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
 */
