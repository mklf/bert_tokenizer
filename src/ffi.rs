use super::FullTokenizer;
use std::ffi::CStr;
use std::os::raw::{c_void,c_char,c_int};
use std::mem;
#[no_mangle]
pub fn create_full_tokenizer(vocab_file:*const c_char,do_lower_case:c_int)->*mut c_void{
    let vocab_file = unsafe { CStr::from_ptr(vocab_file)};
    let vocab_file = vocab_file.to_string_lossy();
    let tokenizer = Box::new(FullTokenizer::new(vocab_file,do_lower_case==1));
    Box::into_raw(tokenizer) as *mut c_void
}

#[no_mangle]
pub fn convert_to_ids(tokenizer:*mut c_void, text:*const c_char,output_len:*mut c_int) ->*mut c_int{
    let tokenizer = unsafe {& *(tokenizer as *mut FullTokenizer)};

    let text =unsafe{ CStr::from_ptr(text)}.to_string_lossy();

    let mut ids:Vec<c_int> = tokenizer.convert_tokens_to_ids(&tokenizer.tokenize(text)).iter().map(|v|*v as _).collect();

    let ids_data = ids.as_mut_ptr();

    unsafe {*output_len = ids.len() as _;}

    mem::forget(ids);
    ids_data
}

#[no_mangle]
pub fn drop_ids(ids_ptr:*mut c_int,len :c_int){
    let ids = unsafe {Vec::from_raw_parts(ids_ptr,len as usize, len as usize)};
}
