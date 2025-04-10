use crate::bridge::bridge;
use crate::capi::plus;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jintArray, jlong, jobjectArray, jsize};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Platform_plus(_: JNIEnv, _: JClass, a: i32, b: i32) -> i32 {
    println!("Loaded from Rust. a = {a}, b = {b}");
    plus(a, b)
}

#[no_mangle]
pub extern "system" fn Java_NativeBridge_newTokenizerFromPretrained(
    mut env: JNIEnv,
    _: JClass,
    identifier: JString,
) -> jlong {
    let id: String = env
        .get_string(&identifier)
        .expect("JNI string conversion failed.")
        .into();

    match bridge::new_tokenizer_from_pretrained(id.as_str()) {
        Ok(ptr) => ptr as jlong,
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_NativeBridge_newTokenizerFromFile(
    mut env: JNIEnv,
    _: JClass,
    filename: JString,
) -> jlong {
    let filename: String = env
        .get_string(&filename)
        .expect("JNI string conversion failed.")
        .into();
    match bridge::new_tokenizer_from_file(filename.as_str()) {
        Ok(ptr) => ptr as jlong,
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_NativeBridge_tokenizerEncode(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    content: JString,
    add_special_tokens: jboolean,
) -> jlong {
    let content: String = env
        .get_string(&content)
        .expect("JNI string conversion failed.")
        .into();
    match bridge::tokenizer_encode(ptr as usize, content.as_str(), add_special_tokens == 1u8) {
        None => {
            env.throw("Null tokenizer pointer.").unwrap();
            0
        }
        Some(Ok(ptr)) => ptr as jlong,
        Some(Err(err)) => {
            env.throw(err.to_string()).unwrap();
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_NativeBridge_encodingGetTokens(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
) -> jobjectArray {
    match bridge::encoding_get_tokens(ptr as usize) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0 as jobjectArray
        }
        Some(tokens) => {
            let string_class = env.find_class("java/lang/String").unwrap();
            let empty_string = env.new_string("").unwrap();
            let array = env
                .new_object_array(tokens.len() as jsize, string_class, empty_string)
                .unwrap();
            for (idx, ele) in tokens.iter().enumerate() {
                env.set_object_array_element(&array, idx as jsize, env.new_string(ele).unwrap())
                    .unwrap()
            }
            array.into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_NativeBridge_encodingGetIds(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
) -> jintArray {
    match bridge::encoding_get_ids(ptr as usize) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0 as jintArray
        }
        Some(ids) => {
            let array = env.new_int_array(ids.len() as jsize).unwrap();
            env.set_int_array_region(
                &array,
                0,
                ids.iter()
                    .map(|id| *id as jint)
                    .collect::<Vec<jint>>()
                    .as_slice(),
            )
            .unwrap();
            array.into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_NativeBridge_encodingGetLen(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong
) -> jint {
    match bridge::encoding_get_len(ptr as usize) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0
        }
        Some(len) => len as jint
    }
}
