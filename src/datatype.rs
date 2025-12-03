use paste::paste;

/// Rust to Matlab data type mapping
pub trait DataType {
    fn mat_type() -> MatType;
    fn to_string() -> String;
}

macro_rules! map {
    ( $( ($rs:ty,$mat:expr) ),+ ) => {
	    $(
        paste! {
            impl DataType for $rs {
            fn mat_type() -> MatType {
                MatType::$mat
            }
            fn to_string() -> String {
                stringify!($rs).to_string()
            }
            }
            impl DataType for &$rs {
            fn mat_type() -> MatType {
                MatType::$mat
            }
            fn to_string() -> String {
                stringify!($rs).to_string()
            }
            }
        }
		)+
    };
}

impl DataType for &str {
    fn mat_type() -> MatType {
        MatType::CHAR
    }

    fn to_string() -> String {
        "&str".into()
    }
}
impl DataType for String {
    fn mat_type() -> MatType {
        MatType::CHAR
    }

    fn to_string() -> String {
        "String".into()
    }
}
impl DataType for Vec<String> {
    fn mat_type() -> MatType {
        MatType::CELL
    }

    fn to_string() -> String {
        "String".into()
    }
}

map! {
(f64, DOUBLE),
(f32, SINGLE),
( i8, INT8),
(i16, INT16),
(i32, INT32),
(i64, INT64),
( u8, UINT8),
(u16, UINT16),
(u32, UINT32),
(u64, UINT64)
}

#[derive(Debug, PartialEq, Eq)]
pub enum MatType {
    DOUBLE,
    SINGLE,
    INT8,
    INT16,
    INT32,
    INT64,
    UINT8,
    UINT16,
    UINT32,
    UINT64,
    STRUCT,
    CHAR,
    CELL,
}

macro_rules! impl_mat_type {
    ( $( ($mat_c:expr,$mat_t:expr) ),+ ) => {
        paste! {
        impl MatType {
            pub fn from_ptr(ptr: *const ffi::matvar_t) -> Option<Self >{
                let mat_ct = unsafe { ((*ptr).class_type, (*ptr).data_type) };
                match mat_ct {
                    $(
                    (ffi::[<matio_classes_MAT_C_ $mat_c>], ffi::[<matio_types_MAT_T_ $mat_t>]) => Some(MatType::$mat_c),
                    )+
                    _ => None
                }
            }
            pub fn to_string(&self) -> String {
                match self {
                    $(
                        MatType::$mat_c => stringify!($mat_c).to_string(),
                    )+
                }
            }
        }
        }
    };
}

impl_mat_type! {
 (DOUBLE,DOUBLE),
 (SINGLE,SINGLE),
 (INT8,INT8),
 (INT16,INT16),
 (INT32,INT32),
 (INT64,INT64),
 (UINT8,UINT8),
 (UINT16,UINT16),
 (UINT32,UINT32),
 (UINT64,UINT64),
 (STRUCT,STRUCT),
 (CHAR,UTF8),
 (CELL,CELL)
}
