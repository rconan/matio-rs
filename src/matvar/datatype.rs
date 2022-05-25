use paste::paste;

// Rust to Matlab data type mapping
pub trait DataType {
    fn mat_c() -> ffi::matio_classes;
    fn mat_t() -> ffi::matio_types;
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
            }
        }
		)+
    };
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
(u64, UINT64),
(crate::MatStruct, STRUCT)
}
