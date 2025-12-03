use paste::paste;

/// Rust to Matlab data type mapping
pub trait DataType {
    fn mat_c() -> ffi::matio_classes;
    fn mat_t() -> ffi::matio_types;
    fn mat_type() -> MatType;
    fn to_string() -> String;
}

macro_rules! map {
    ( $( ($rs:ty,$mat:expr) ),+ ) => {
	    $(
        paste! {
            impl DataType for $rs {
            fn mat_c() -> ffi::matio_classes {
                        ffi::[<matio_classes_MAT_C_ $mat>]
            }
            fn mat_t() -> ffi::matio_types {
                        ffi::[<matio_types_MAT_T_ $mat>]
            }
            fn mat_type() -> MatType {
                MatType::$mat
            }
            fn to_string() -> String {
                stringify!($rs).to_string()
            }
            }
            impl DataType for &$rs {
            fn mat_c() -> ffi::matio_classes {
                        ffi::[<matio_classes_MAT_C_ $mat>]
            }
            fn mat_t() -> ffi::matio_types {
                        ffi::[<matio_types_MAT_T_ $mat>]
            }
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
    fn mat_c() -> ffi::matio_classes {
        ffi::matio_classes_MAT_C_CHAR
    }

    fn mat_t() -> ffi::matio_types {
        ffi::matio_types_MAT_T_UINT8
    }

    fn mat_type() -> MatType {
        MatType::UINT8
    }

    fn to_string() -> String {
        "str".into()
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
}

macro_rules! impl_mat_type {
    ( $( $mat:expr ),+ ) => {
        paste! {
        impl MatType {
            pub fn from_ptr(ptr: *const ffi::matvar_t) -> Option<Self >{
                let mat_ct = unsafe { ((*ptr).class_type, (*ptr).data_type) };
                match mat_ct {
                    $(
                    (ffi::[<matio_classes_MAT_C_ $mat>], ffi::[<matio_types_MAT_T_ $mat>]) => Some(MatType::$mat),
                    )+
                    _ => None
                }
            }
            pub fn to_string(&self) -> String {
                match self {
                    $(
                        MatType::$mat => stringify!($mat).to_string(),
                    )+
                }
            }
        }
        }
    };
}

impl_mat_type! {
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
 STRUCT
}
