extern crate git2;

use git2::Repository;
use std::env::current_dir;
use std::mem::uninitialized;
use std::ffi::CString;
use std::ptr;

// Provides a space-delimited list of symbols provided by this library.
#[no_mangle]
pub extern "C" fn index() -> *const u8 {
    b"branch modified_count staged_count ahead_count behind_count\0".as_ptr()
}


#[no_mangle]
pub extern "C" fn branch() -> *mut i8 {
    if let Ok(dir) = current_dir() {
        if let Ok(repo) = Repository::open(dir) {
            if let Ok(head) = repo.head() {
                if let Some(name) = head.shorthand() {
                    let string = CString::new(name.as_bytes()).unwrap();
                    return string.into_raw();
                }
            }
        }
    }

    ptr::null_mut()
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

// Build *mut i8 from a String
fn build_return_value(s: String) -> *mut i8 {
    let string = CString::new(s.as_bytes()).unwrap();
    string.into_raw()
}

// There is a common theme among functions such as changed_count, staged_count, etc.
// This function does the common part
fn count_statuses(status: git2::Status) -> *mut i8 {
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
pub extern "C" fn modified_count() -> *mut i8 {
    use git2::*;

    let modified = STATUS_WT_DELETED | STATUS_WT_MODIFIED | STATUS_WT_RENAMED | STATUS_WT_TYPECHANGE;
    count_statuses(modified)
}


#[no_mangle]
pub extern "C" fn staged_count() -> *mut i8 {
    use git2::*;

    let staged =  STATUS_INDEX_DELETED | STATUS_INDEX_MODIFIED | STATUS_INDEX_NEW |
                  STATUS_INDEX_RENAMED | STATUS_INDEX_TYPECHANGE;
    count_statuses(staged)
}

fn count_revs_between_branches(from: &str, to: &str) -> *mut i8 {
    use git2::*;

    if let Ok(dir) = current_dir() {
        if let Ok(repo) = Repository::open(dir) {
            if let Ok(mut revwalk) = repo.revwalk() {
                if let Ok(_) = revwalk.push_range(&format!("{}..{}", from, to)) {
                    let count = revwalk.count();
                    return build_return_value(format!("{}", count));
                }
            }
        }
    }

    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ahead_count() -> *mut i8 {
    count_revs_between_branches("@{u}", "HEAD")
}

#[no_mangle]
pub extern "C" fn behind_count() -> *mut i8 {
    count_revs_between_branches("HEAD", "@{u}")
}
