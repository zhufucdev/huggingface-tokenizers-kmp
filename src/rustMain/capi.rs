use crate::bridge::bridge;
use std::alloc;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::fmt::Debug;

#[no_mangle]
pub extern "C" fn plus(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn new_tokenizer_from_pretrained(
    identifier: *const c_char,
) -> Result<*const c_void> {
    match bridge::new_tokenizer_from_pretrained(
        CStr::from_ptr(identifier)
            .to_str()
            .expect("FFI string conversion failed."),
    ) {
        Ok(ptr) => Result::ok(ptr as *const c_void),
        Err(err) => Result::error_to_str_nilptr(err),
    }
}

#[no_mangle]
pub unsafe extern "C" fn new_tokenizer_from_file(filename: *const c_char) -> Result<*const c_void> {
    match bridge::new_tokenizer_from_file(
        CStr::from_ptr(filename)
            .to_str()
            .expect("FFI string conversion failed."),
    ) {
        Ok(ptr) => Result::ok(ptr as *const c_void),
        Err(err) => Result::error_to_str_nilptr(err),
    }
}

#[no_mangle]
pub extern "C" fn tokenizer_encode(
    ptr: *const c_void,
    content: *const c_char,
    add_special_tokens: bool,
) -> Result<*const c_void> {
    let content = unsafe {
        CStr::from_ptr(content)
            .to_str()
            .expect("FFI string conversion failed.")
    };
    match bridge::tokenizer_encode(ptr as usize, content, add_special_tokens) {
        None => Result::error_nilptr("Nil tokenizer pointer."),
        Some(Ok(ptr)) => Result::ok(ptr as *const c_void),
        Some(Err(err)) => Result::error_to_str_nilptr(err),
    }
}

#[no_mangle]
pub unsafe extern "C" fn release_cstring_ptr(ptr: *mut c_char) {
    _ = CString::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_tokens(ptr: *const c_void) -> Result<List<*const c_char>> {
    match bridge::encoding_get_tokens(ptr as usize) {
        None => Result::error_empty("Nil encoding pointer."),
        Some(tokens) => Result::ok(List::from_vec(
            tokens
                .iter()
                .map(|token| CString::new(token.as_str()).unwrap().into_raw() as *const c_char)
                .collect(),
        )),
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_ids(ptr: *const c_void) -> Result<List<c_uint>> {
    match bridge::encoding_get_ids(ptr as usize) {
        None => Result::error_empty("Nil encoding pointer."),
        Some(ids) => Result::ok(List::from_vec(ids)),
    }
}

#[no_mangle]
pub extern "C" fn encoding_get_len(ptr: *const c_void) -> Result<usize> {
    match bridge::encoding_get_len(ptr as usize) {
        None => Result::error(0, "Nil encoding pointer."),
        Some(len) => Result::ok(len)
    }
}

#[repr(C)]
pub struct Result<T> {
    value: T,
    error_msg: *const c_char,
}

impl<T> Result<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value,
            error_msg: 0 as *const c_char,
        }
    }

    pub fn error<E: Debug>(default: T, error: E) -> Self {
        let msg = CString::new(format!("{:?}", error)).unwrap();
        Self {
            value: default,
            error_msg: msg.into_raw(),
        }
    }

    pub fn error_to_str<E: ToString>(default: T, error: E) -> Self {
        let msg = CString::new(error.to_string()).unwrap();
        Self {
            value: default,
            error_msg: msg.into_raw(),
        }
    }
}

impl Result<*const c_void> {
    pub fn error_nilptr<E: Debug>(error: E) -> Self {
        Self::error(0 as *const c_void, error)
    }

    pub fn error_to_str_nilptr<E: ToString>(error: E) -> Self {
        Self::error_to_str(0 as *const c_void, error)
    }
}

impl<T> Result<List<T>> {
    pub fn error_empty<E: Debug>(error: E) -> Self {
        Self::error(List::empty(), error)
    }

    pub fn error_to_str_empty<E: ToString>(error: E) -> Self {
        Self::error_to_str(List::empty(), error)
    }
}

#[repr(C)]
pub struct List<T> {
    ptr: *const T,
    len: usize,
}

impl<T> List<T> {
    pub fn empty() -> Self {
        Self {
            ptr: 0 as *const T,
            len: 0,
        }
    }
}

impl<T: Clone> List<T> {
    pub unsafe fn from_vec(vec: Vec<T>) -> Self {
        let layout = alloc::Layout::from_size_align(vec.len(), size_of::<T>()).unwrap();
        let span = alloc::alloc_zeroed(layout) as *mut T;
        for (idx, ele) in vec.iter().enumerate() {
            let owned = ele.clone();
            span.offset(idx as isize).write(owned)
        }
        Self {
            ptr: span,
            len: vec.len(),
        }
    }
}
