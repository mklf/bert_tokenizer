use super::Judger;
use super::{ATTENTION_MASK, ERROR_MSG, INPUT_IDS, TOKEN_TYPE_IDS};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;

#[no_mangle]
pub fn judger_get_error() -> *const c_char {
    let error_ptr = ERROR_MSG.with(|error_msg| {
        return error_msg.borrow().as_ptr();
    });
    error_ptr
}

#[no_mangle]
pub fn create_judger(filename: *const c_char) -> *mut c_void {
    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = filename.to_string_lossy();
    match Judger::new(filename) {
        Ok(judger) => {
            let judger = Box::new(judger);
            Box::into_raw(judger) as *mut c_void
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
pub fn drop_judger(judger: *mut c_void) {
    unsafe { Box::from_raw(judger) };
}

#[no_mangle]
pub fn judger_process(
    judger: *mut c_void,
    texts: *const *const c_char,
    input_size: u32,
    max_length: u32,
) -> i64 {
    static CONTEXT_SEP: i32 = 99;
    let judger = unsafe { &*(judger as *mut Judger) };
    if input_size == 0 || input_size as usize != judger.input_size() {
        ERROR_MSG.with(|error_msg| {
            let mut error_msg = error_msg.borrow_mut();
            let reason = format!(
                "input_size mismatch, expected val is {}",
                judger.input_size()
            )
            .into_bytes();
            *error_msg = CString::new(reason).unwrap();
        });
        return -2;
    }

    let query =
        unsafe { CStr::from_ptr(*(texts.offset(input_size as isize - 1))) }.to_string_lossy();
    if query.len() == 0 {
        ERROR_MSG.with(|error_msg| {
            let mut error_msg = error_msg.borrow_mut();
            let reason = "query is empty".to_owned().into_bytes();
            *error_msg = CString::new(reason).unwrap();
        });
        return -2;
    }

    let val = judger.get(query.as_ref());
    if val != -1 {
        return val;
    }

    let max_length = max_length as usize;
    let mut ids = vec![];
    for i in 0..input_size as isize {
        let text = unsafe { CStr::from_ptr(*(texts.offset(i))) }.to_string_lossy();
        let text = judger.format(&text, i as usize);
        ids.push(judger.tokenizer.tokenize_to_ids(text));
    }
    ids.push(judger.tokenizer.tokenize_to_ids(judger.prompt()));
    let mut context_input_ids: Vec<i32> = vec![];
    let mut current: Vec<i32> = vec![];
    if input_size > 0 {
        let ids_length = ids.len();
        let (context, current_) = ids.split_at_mut(ids_length - 1);
        current.append(&mut current_[0]);

        for c in context {
            context_input_ids.append(c);
            context_input_ids.push(CONTEXT_SEP);
        }
        if current.len() >= max_length {
            ERROR_MSG.with(|error_msg| {
                let mut error_msg = error_msg.borrow_mut();
                let reason = "max_length to small".to_owned().into_bytes();
                *error_msg = CString::new(reason).unwrap();
            });
            return -2;
        }

        let context_max_length = max_length - current.len() - 1;
        if context_input_ids.len() > context_max_length {
            let len = context_input_ids.len();
            context_input_ids = context_input_ids
                .split_at_mut(len - context_max_length)
                .1
                .to_vec();
        }
        if let Some(last) = context_input_ids.last_mut() {
            *last = judger.tokenizer.sep_token_id as _;
        }
    }

    TOKEN_TYPE_IDS.with(|token_type_ids| {
        let mut token_type_ids = token_type_ids.borrow_mut();
        token_type_ids.clear();
        token_type_ids.resize(context_input_ids.len() + 1, 0);
        token_type_ids.append(&mut vec![1; current.len()]);
        token_type_ids.resize(max_length, 0);
    });

    ATTENTION_MASK.with(|attention_mask| {
        let mut attention_mask = attention_mask.borrow_mut();
        attention_mask.clear();
        attention_mask.resize(context_input_ids.len() + current.len() + 1, 1);
        attention_mask.resize(max_length, 0);
    });

    INPUT_IDS.with(|input_ids| {
        let mut input_ids = input_ids.borrow_mut();
        input_ids.clear();
        input_ids.push(judger.tokenizer.cls_token_id as _);
        input_ids.append(&mut context_input_ids);
        input_ids.append(&mut current);
        input_ids.resize(max_length, judger.tokenizer.pad_token_id as _);
    });

    return -1;
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
pub fn get_attention_mask() -> *mut i32 {
    let mut attention_mask_ptr: *mut i32 = ptr::null_mut();
    ATTENTION_MASK.with(|attention_mask| {
        attention_mask_ptr = attention_mask.borrow_mut().as_mut_ptr();
    });
    attention_mask_ptr
}

#[no_mangle]
pub fn get_token_type_ids() -> *mut i32 {
    let mut token_type_ids_ptr: *mut i32 = ptr::null_mut();
    TOKEN_TYPE_IDS.with(|token_type_ids| {
        token_type_ids_ptr = token_type_ids.borrow_mut().as_mut_ptr();
    });
    token_type_ids_ptr
}

mod test {
    use super::*;
    #[test]
    fn test_ok() {
        unsafe {
            let cs = CString::new(
                "/home/svn/frankfangli/code/rust/bert_tokenizer/ffi/x.json".as_bytes(),
            )
            .unwrap();
            let handle = create_judger(cs.as_ptr());

            let a = CString::new("我不知道那是我的孩子他们说买我看一下的".as_bytes()).unwrap();
            let b = CString::new("咋了啊他了不了了".as_bytes()).unwrap();

            let texts = [a.as_ptr(), b.as_ptr()];

            judger_process(handle, texts.as_ptr(), 2, 128);
        }
    }
}
