use crate::bridge::bridge;
use std::alloc;
use std::alloc::{Layout, LayoutError};
use std::ffi::{c_char, c_int, c_void, CStr, CString};
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
    input: *const c_char,
    add_special_tokens: bool,
) -> Result<*const c_void> {
    let input = unsafe {
        CStr::from_ptr(input)
            .to_str()
            .expect("FFI string conversion failed.")
    };
    match bridge::tokenizer_encode(ptr as usize, input, add_special_tokens) {
        None => Result::error_nilptr("Nil tokenizer pointer."),
        Some(Ok(ptr)) => Result::ok(ptr as *const c_void),
        Some(Err(err)) => Result::error_to_str_nilptr(err),
    }
}

#[no_mangle]
pub unsafe extern "C" fn tokenizer_encode_batch(
    ptr: *const c_void,
    inputs: *const *const c_char,
    input_count: c_int,
    add_special_tokens: bool,
) -> Result<List> {
    let inputs = {
        let mut vec = Vec::with_capacity(input_count as usize);
        for i in 0..input_count {
            match inputs.offset(i as isize).as_ref() {
                None => return Result::error_empty(format!("Nil pointer at input index {}", i)),
                Some(input) => vec.push(
                    CStr::from_ptr(*input)
                        .to_str()
                        .expect("FFI string conversion failed."),
                ),
            }
        }
        vec
    };
    match bridge::tokenizer_encode_batch(ptr as usize, inputs, add_special_tokens) {
        None => Result::error_empty("Nil tokenizer pointer."),
        Some(Ok(pointers)) => Result::ok(pointers.iter().map(|p| *p as *const c_void).collect()),
        Some(Err(err)) => Result::error_to_str_empty(err),
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_tokens(ptr: *const c_void) -> Result<List> {
    match bridge::encoding_get_tokens(&(ptr as usize)) {
        None => Result::error_empty("Nil encoding pointer."),
        Some(tokens) => Result::ok(
            tokens
                .iter()
                .map(|token| CString::new(token.as_str()).unwrap().into_raw() as *const c_char)
                .collect(),
        ),
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_ids(ptr: *const c_void) -> Result<List> {
    match bridge::encoding_get_ids(&(ptr as usize)) {
        None => Result::error_empty("Nil encoding pointer."),
        Some(ids) => Result::ok(ids.into()),
    }
}

#[no_mangle]
pub extern "C" fn encoding_get_len(ptr: *const c_void) -> Result<usize> {
    match bridge::encoding_get_len(&(ptr as usize)) {
        None => Result::error(0, "Nil encoding pointer."),
        Some(len) => Result::ok(len),
    }
}

#[no_mangle]
pub extern "C" fn encoding_eq(ptr: *const c_void, other_ptr: *const c_void) -> Result<bool> {
    match bridge::encoding_eq(&(ptr as usize), &(other_ptr as usize)) {
        None => Result::error(false, "Nil encoding pointer."),
        Some(eq) => Result::ok(eq)
    }
}

#[no_mangle]
pub extern "C" fn release_tokenizer(ptr: *mut c_void) {
    bridge::release_tokenizer(ptr as usize)
}

#[no_mangle]
pub extern "C" fn release_encoding(ptr: *mut c_void) {
    bridge::release_encoding(ptr as usize)
}

#[no_mangle]
pub unsafe extern "C" fn release_cstring_ptr(ptr: *mut c_char) {
    _ = CString::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn release_list(list: List, align: usize) {
    list.dealloc_align(align).unwrap()
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

impl<T> Result<*const T> {
    pub fn error_nilptr<E: Debug>(error: E) -> Self {
        Self::error(0 as *const T, error)
    }

    pub fn error_to_str_nilptr<E: ToString>(error: E) -> Self {
        Self::error_to_str(0 as *const T, error)
    }
}

impl Result<List> {
    pub fn error_empty<E: Debug>(error: E) -> Self {
        Self::error(List::empty(), error)
    }

    pub fn error_to_str_empty<E: ToString>(error: E) -> Self {
        Self::error_to_str(List::empty(), error)
    }
}

#[repr(C)]
pub struct List {
    ptr: *const u8,
    len: usize,
}

impl List {
    pub fn empty() -> Self {
        Self {
            ptr: 0 as *const u8,
            len: 0,
        }
    }

    pub unsafe fn dealloc<T>(self) -> std::result::Result<(), LayoutError> {
        alloc::dealloc(self.ptr as *mut u8, self.layout::<T>()?);
        Ok(())
    }
    
    pub unsafe fn dealloc_align(self, align: usize) -> std::result::Result<(), LayoutError> {
        alloc::dealloc(self.ptr as *mut u8, Layout::from_size_align(self.len, align)?);
        Ok(())
    }

    fn layout<T>(&self) -> std::result::Result<Layout, LayoutError> {
        Layout::from_size_align(self.len, size_of::<T>())
    }
}

impl List {
    pub unsafe fn from_vec<T>(vec: Vec<T>) -> Self {
        let len = vec.len();
        let layout = Layout::from_size_align(len, size_of::<T>()).unwrap();
        let span = alloc::alloc_zeroed(layout) as *mut T;
        for (idx, ele) in vec.into_iter().enumerate() {
            span.offset(idx as isize).write(ele)
        }
        Self { ptr: span as *mut u8, len }
    }
}

impl<T> FromIterator<T> for List {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec = Vec::from_iter(iter);
        unsafe { Self::from_vec(vec) }
    }
}

impl<T: Clone> From<&[T]> for List {
    fn from(value: &[T]) -> Self {
        let layout = Layout::from_size_align(value.len(), size_of::<T>()).unwrap();
        let span = unsafe { alloc::alloc_zeroed(layout) as *mut T };
        for (idx, ele) in value.iter().enumerate() {
            let owned = ele.clone();
            unsafe { span.offset(idx as isize).write(owned) }
        }
        Self {
            ptr: span as *mut u8,
            len: value.len(),
        }
    }
}
