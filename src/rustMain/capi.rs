use crate::bridge::bridge;
use std::alloc::{Layout, LayoutError};
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::fmt::Debug;
use std::{alloc, slice};

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
pub unsafe extern "C" fn new_tokenizer_from_bytes(
    bytes: *mut c_char,
    length: c_int,
) -> Result<*const c_void> {
    if bytes.is_null() {
        return Result::error_nilptr("Nil bytes pointer.");
    }
    let buff = slice::from_raw_parts(bytes as *const u8, length as usize);
    match bridge::new_tokenizer_from_bytes(buff) {
        Ok(ptr) => Result::ok(ptr as *const c_void),
        Err(err) => Result::error_to_str_nilptr(err),
    }
}

#[no_mangle]
pub unsafe extern "C" fn tokenizer_encode(
    ptr: *const c_void,
    input: *const c_char,
    add_special_tokens: bool,
) -> Result<*const c_void> {
    let input = CStr::from_ptr(input)
        .to_str()
        .expect("FFI string conversion failed.");
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
pub unsafe extern "C" fn encoding_get_token_at(
    ptr: *const c_void,
    index: usize,
) -> Result<*const c_char> {
    match bridge::encoding_get_token_at(&(ptr as usize), index) {
        None => Result::error_nilptr("Nil encoding pointer."),
        Some(token) => match CString::new(token.as_str()) {
            Ok(str) => Result::ok(CString::into_raw(str)),
            Err(err) => Result::error_nilptr(format!("Rust-to-C string conversion failed: {err}")),
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_id_at(ptr: *const c_void, index: usize) -> Result<u32> {
    match bridge::encoding_get_id_at(&(ptr as usize), index) {
        None => Result::error_default("Nil encoding pointer."),
        Some(ids) => Result::ok(ids.into()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_sequence_id_at(ptr: *const c_void, index: usize) -> Result<usize> {
    match bridge::encoding_get_sequence_id_at(&(ptr as usize), index) {
        None => Result::error_default("Nil encoding pointer."),
        Some(Some(id)) => Result::ok(id + 1),
        Some(None) => Result::ok(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_attention_mask_at(ptr: *const c_void, index: usize) -> Result<u32> {
    match bridge::encoding_get_attention_mask_at(&(ptr as usize), index) {
        None => Result::error_default("Nil encoding pointer."),
        Some(id) => Result::ok(id),
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_get_len(ptr: *const c_void) -> Result<usize> {
    match bridge::encoding_get_len(&(ptr as usize)) {
        None => Result::error(0, "Nil encoding pointer."),
        Some(len) => Result::ok(len),
    }
}

#[no_mangle]
pub unsafe extern "C" fn encoding_eq(ptr: *const c_void, other_ptr: *const c_void) -> Result<bool> {
    match bridge::encoding_eq(&(ptr as usize), &(other_ptr as usize)) {
        None => Result::error(false, "Nil encoding pointer."),
        Some(eq) => Result::ok(eq),
    }
}

#[no_mangle]
pub unsafe extern "C" fn release_tokenizer(ptr: *mut c_void) {
    bridge::release_tokenizer(ptr as usize)
}

#[no_mangle]
pub unsafe extern "C" fn release_encoding(ptr: *mut c_void) {
    bridge::release_encoding(ptr as usize)
}

#[no_mangle]
pub unsafe extern "C" fn release_cstring_ptr(ptr: *mut c_char) {
    drop(CString::from_raw(ptr));
}

#[no_mangle]
pub unsafe extern "C" fn release_list(list: List, size: usize, align: usize) {
    list.dealloc_align(size, align).unwrap()
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
    
    pub fn error_default<E: Debug>(error: E) -> Self where T: Default {
        Self::error(T::default(), error)
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
        if !self.ptr.is_null() {
            alloc::dealloc(self.ptr as *mut u8, self.layout::<T>()?);
        }
        Ok(())
    }

    pub unsafe fn dealloc_align(
        self,
        size: usize,
        align: usize,
    ) -> std::result::Result<(), LayoutError> {
        if !self.ptr.is_null() {
            alloc::dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align(self.len * size, align)?,
            );
        }
        Ok(())
    }

    fn layout<T>(&self) -> std::result::Result<Layout, LayoutError> {
        Layout::array::<T>(self.len)
    }
}

impl List {
    pub unsafe fn from_vec<T>(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            return Self::empty();
        }
        let len = vec.len();
        let layout = Layout::array::<T>(len).unwrap();
        let span = alloc::alloc_zeroed(layout) as *mut T;
        for (idx, ele) in vec.into_iter().enumerate() {
            span.offset(idx as isize).write(ele)
        }
        Self {
            ptr: span as *mut u8,
            len,
        }
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
        if value.is_empty() {
            return Self::empty();
        }
        let layout = Layout::array::<T>(value.len()).unwrap();
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
