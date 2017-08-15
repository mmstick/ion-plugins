use std::ffi::CString;

// Provides a space-delimited list of symbols provided by this library.
#[no_mangle]
pub extern "C" fn index() -> *const u8 {
    b"test\0".as_ptr()
}

/// Either one or the other will be set. Optional status can be conveyed by setting the
/// corresponding field to `NULL`. Libraries importing this structure should check for nullness.
#[repr(C)]
pub struct RawMethodArguments {
    key_ptr: *mut i8,
    key_array_ptr: *mut *mut i8,
    args_ptr: *mut *mut i8,
    key_len: usize,
    args_len: usize,
}

pub enum MethodArguments {
    StringArg(String, Vec<String>),
    Array(Vec<String>, Vec<String>),
    NoArgs
}

impl From<RawMethodArguments> for MethodArguments {
    fn from(input: RawMethodArguments) -> MethodArguments {
        if input.key_len == 0 {
            return MethodArguments::NoArgs
        }

        if !input.key_ptr.is_null() {
            unsafe {
                let key = CString::from_raw(input.key_ptr);
                let args = Vec::from_raw_parts(input.args_ptr, input.args_len, input.args_len);
                return MethodArguments::StringArg(
                    key.into_string().unwrap(),
                    args.iter().map(|&s| CString::from_raw(s).into_string().unwrap())
                        .collect::<Vec<String>>()
                );
            }
        } else if !input.key_array_ptr.is_null() {
            unsafe {
                let key = Vec::from_raw_parts(input.key_array_ptr, input.key_len, input.key_len);
                let args = Vec::from_raw_parts(input.args_ptr, input.args_len, input.args_len);
                return MethodArguments::Array(
                    key.iter().map(|&s| CString::from_raw(s).into_string().unwrap())
                        .collect::<Vec<String>>(),
                    args.iter().map(|&s| CString::from_raw(s).into_string().unwrap())
                        .collect::<Vec<String>>()
                )
            }
        }

        MethodArguments::NoArgs
    }
}

#[no_mangle]
pub extern "C" fn test(arguments: RawMethodArguments) -> *mut i8  {
    let result = match MethodArguments::from(arguments) {
        MethodArguments::StringArg(string, args) => {
            format!("key: String({}); args: {:?}", string, args)
        },
        MethodArguments::NoArgs => {
            format!("no arguments supplied")
        },
        MethodArguments::Array(array, args) => {
            format!("key: Array({:?}); args: {:?}", array, args)
        }
    };

    CString::new(result).unwrap().into_raw()
}
