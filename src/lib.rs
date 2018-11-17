extern crate unicode_normalization;
extern crate unicode_categories;
extern crate indexmap;
use std::cell::RefCell;
use std::os::raw::c_int;
use std::ffi::CString;


thread_local!{
    pub static INPUT_IDS : RefCell<Vec<c_int>> = RefCell::new(Vec::new());
    pub static INPUT_MASK: RefCell<Vec<c_int>> = RefCell::new(Vec::new());
    pub static SEGMENT_IDS:RefCell<Vec<c_int>> = RefCell::new(Vec::new());
    pub static ERROR_MSG : RefCell<CString> = RefCell::new(CString::new("").unwrap());
}


mod tokenization;
mod ffi;

pub use self::tokenization::*;

pub use ffi::*;
