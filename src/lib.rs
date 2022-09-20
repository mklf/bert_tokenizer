extern crate indexmap;
extern crate unicode_categories;
extern crate unicode_normalization;
use std::cell::RefCell;
use std::ffi::CString;

thread_local! {
    pub static INPUT_IDS : RefCell<Vec<i32>> = RefCell::new(Vec::new());
    pub static INPUT_MASK: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    pub static SEGMENT_IDS:RefCell<Vec<i32>> = RefCell::new(Vec::new());
    pub static ERROR_MSG : RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

mod ffi;
mod tokenization;

pub use self::tokenization::*;

pub use ffi::*;
