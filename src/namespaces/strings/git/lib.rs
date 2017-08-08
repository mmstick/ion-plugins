extern crate git2;

use git2::Repository;
use std::env::current_dir;
use std::mem::uninitialized;
use std::ffi::CString;

#[repr(C)]
pub struct NamespaceResult {
    /// Designates whether there is `Some` value or not.
    exists: bool,
    /// The binary stream returned by the function
    data: *mut i8,
}

// Provides a space-delimited list of symbols provided by this library.
#[no_mangle]
pub extern "C" fn index() -> *const u8 {
    b"branch\0".as_ptr()
}


#[no_mangle]
pub extern "C" fn branch() -> NamespaceResult {
    if let Ok(dir) = current_dir() {
        if let Ok(repo) = Repository::open(dir) {
            if let Ok(head) = repo.head() {
                if let Some(name) = head.shorthand() {
                    let string = CString::new(name.as_bytes()).unwrap();
                    let data = string.into_raw();
                    return NamespaceResult { exists: true, data };
                }
            }
        }
    }

    NamespaceResult {
        exists: false,
        data: unsafe { uninitialized() },
    }
}
