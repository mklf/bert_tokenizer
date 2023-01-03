extern crate indexmap;
extern crate unicode_categories;
extern crate unicode_normalization;
extern crate serde_json;
extern crate serde;
use std::cell::RefCell;
use std::ffi::CString;

thread_local! {
    pub static INPUT_IDS : RefCell<Vec<i32>> = RefCell::new(Vec::new());
    pub static ATTENTION_MASK: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    pub static TOKEN_TYPE_IDS:RefCell<Vec<i32>> = RefCell::new(Vec::new());
    pub static ERROR_MSG : RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

mod ffi;
mod tokenization;
mod judger;
pub use self::tokenization::*;
pub use self::judger::*;

pub use ffi::*;
