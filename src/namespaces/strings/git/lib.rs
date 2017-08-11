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
    b"branch modified_count staged_count\0".as_ptr()
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

// Iterates over statuses of all files in a repository
fn iterate_statuses<F>(mut iterator_fn: F)
    where F: FnMut(&git2::StatusEntry) {
    if let Ok(dir) = current_dir() {
        if let Ok(repo) = Repository::open(dir) {
            if let Ok(statuses) = repo.statuses(None) {
                for i in statuses.iter() {
                    iterator_fn(&i);
                }
            }
        }
    }
}

// Build NamespaceResult from a String
fn build_return_value(s: String) -> NamespaceResult {
    let string = CString::new(s.as_bytes()).unwrap();
    let data = string.into_raw();
    return NamespaceResult { exists: true, data };
}

// There is a common theme among functions such as changed_count, staged_count, etc.
// This function does the common part
fn count_statuses(status: git2::Status) -> NamespaceResult {
    let mut acc = 0;
    iterate_statuses(|i| {
        if i.status().intersects(status) {
            acc += 1;
        }
    });

    let ret_string: String = format!("{}", acc);
    build_return_value(ret_string)
}

#[no_mangle]
pub extern "C" fn modified_count() -> NamespaceResult {
    use git2::*;

    let modified = STATUS_WT_DELETED | STATUS_WT_MODIFIED | STATUS_WT_RENAMED | STATUS_WT_TYPECHANGE;
    count_statuses(modified)
}


#[no_mangle]
pub extern "C" fn staged_count() -> NamespaceResult {
    use git2::*;

    let staged =  STATUS_INDEX_DELETED | STATUS_INDEX_MODIFIED | STATUS_INDEX_NEW |
                  STATUS_INDEX_RENAMED | STATUS_INDEX_TYPECHANGE;
    count_statuses(staged)
}