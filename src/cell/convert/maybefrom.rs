use std::{collections::VecDeque, ffi::CString};

use crate::{
    Mat, MatioError, MayBeFrom, MayBeInto, Result,
    cell::{Cell, CellBounds},
};

impl<'a, T, C> MayBeFrom<Cell<T, C>> for Mat<'a>
where
    C: CellBounds,
    for<'b> Mat<'b>: MayBeFrom<T> + MayBeInto<T>,
{
    fn maybe_from<S: Into<String>>(name: S, cell: Cell<T, C>) -> Result<Self> {
        let mut mat = VecDeque::try_from(cell)?;
        mat.iter_mut().for_each(|mat| {
            mat.as_ref = true;
        });
        let c_name = CString::new(name.into())?;
        let mut dims = [1, mat.len()];
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
        for (i, mat) in mat.into_iter().enumerate() {
            unsafe {
                ffi::Mat_VarSetCell(matcell_t, i as i32, mat.matvar_t);
            }
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
