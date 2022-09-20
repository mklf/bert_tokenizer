use super::{FullTokenizer, ERROR_MSG, INPUT_IDS, INPUT_MASK, SEGMENT_IDS};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

#[no_mangle]
pub fn bert_tokenizer_get_error() -> *const c_char {
    let error_ptr = ERROR_MSG.with(|error_msg| {
        return error_msg.borrow().as_ptr();
    });
    error_ptr
}

#[no_mangle]
pub fn create_full_tokenizer(vocab_file: *const c_char, do_lower_case: c_int) -> *mut c_void {
    let vocab_file = unsafe { CStr::from_ptr(vocab_file) };
    let vocab_file = vocab_file.to_string_lossy();
    match FullTokenizer::new(vocab_file, do_lower_case == 1) {
        Ok(tokenizer) => {
            let tokenizer = Box::new(tokenizer);
            Box::into_raw(tokenizer) as *mut c_void
        }
        Err(e) => {
            ERROR_MSG.with(|error_msg| {
                let mut error_msg = error_msg.borrow_mut();
                let reason = e.to_string().into_bytes();
                *error_msg = CString::new(reason).unwrap();
            });
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub fn drop_tokenizer(tokenizer: *mut c_void) {
    unsafe { Box::from_raw(tokenizer) };
}

#[no_mangle]
pub fn convert_to_ids(
    tokenizer: *mut c_void,
    text: *const c_char,
    output_len: *mut c_int,
) -> *mut c_int {
    let tokenizer = unsafe { &*(tokenizer as *mut FullTokenizer) };

    let text = unsafe { CStr::from_ptr(text) }.to_string_lossy();

    let mut ids: Vec<c_int> = tokenizer
        .convert_tokens_to_ids(&tokenizer.tokenize(text))
        .iter()
        .map(|v| *v as _)
        .collect();

    let ids_data = ids.as_mut_ptr();

    unsafe {
        *output_len = ids.len() as _;
    }

    mem::forget(ids);
    ids_data
}

#[no_mangle]
pub fn drop_ids(ids_ptr: *mut c_int, len: c_int) {
    let _ids = unsafe { Vec::from_raw_parts(ids_ptr, len as usize, len as usize) };
}

#[no_mangle]
pub fn convert_pairs(
    tokenizer: *mut c_void,
    text_a: *const c_char,
    text_b: *const c_char,
    max_seq_len: c_int,
    is_pair: c_int,
) -> c_int {
    let tokenizer = unsafe { &*(tokenizer as *mut FullTokenizer) };

    let text_a = unsafe { CStr::from_ptr(text_a) }.to_string_lossy();
    let text_b = unsafe { CStr::from_ptr(text_b) }.to_string_lossy();


    let seq_len = tokenizer.convert_pairs(
        text_a.as_ref(),
        text_b.as_ref(),
        max_seq_len as usize,
        is_pair == 1,
    );
    seq_len as c_int
}

#[no_mangle]
pub fn get_input_ids() -> *mut i32 {
    let mut input_ids_ptr: *mut i32 = ptr::null_mut();
    INPUT_IDS.with(|input_ids| {
        input_ids_ptr = input_ids.borrow_mut().as_mut_ptr();
    });
    input_ids_ptr
}

#[no_mangle]
pub fn get_input_mask() -> *mut i32 {
    let mut input_mask_ptr: *mut i32 = ptr::null_mut();
    INPUT_MASK.with(|input_mask| {
        input_mask_ptr = input_mask.borrow_mut().as_mut_ptr();
    });
    input_mask_ptr
}

#[no_mangle]
pub fn get_segment_ids() -> *mut i32 {
    let mut segment_ids_ptr: *mut i32 = ptr::null_mut();
    SEGMENT_IDS.with(|segment_ids| {
        segment_ids_ptr = segment_ids.borrow_mut().as_mut_ptr();
    });
    segment_ids_ptr
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pipeline() {
        let tokenizer = FullTokenizer::new("vocab.txt", true).unwrap();
        let len = tokenizer.convert_pairs("你好帅","你好你好你好", 0, true);
        unsafe {
            let input_ids = std::slice::from_raw_parts(get_input_ids(), len);
            let token_type_ids = std::slice::from_raw_parts(get_segment_ids(), len);

            println!("{:?}", input_ids);
            println!("{:?}", INPUT_IDS);
            println!("{:?}", token_type_ids);
        }
    }
}
