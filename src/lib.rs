extern crate unicode_normalization;
extern crate unicode_categories;
extern crate indexmap;
use std::cell::RefCell;
use std::os::raw::c_int;
thread_local!{
    pub static INPUT_IDS : RefCell<Vec<c_int>> = RefCell::new(Vec::new());
    pub static INPUT_MASK: RefCell<Vec<c_int>> = RefCell::new(Vec::new());
    pub static SEGMENT_IDS:RefCell<Vec<c_int>> = RefCell::new(Vec::new());

}


mod tokenization;
mod ffi;

pub use self::tokenization::*;

pub use ffi::create_full_tokenizer;
pub use ffi::convert_to_ids;
pub use ffi::drop_ids;