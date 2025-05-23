use crate::bridge::bridge;
use crate::capi::plus;
use jni::objects::{JByteArray, JClass, JObjectArray, JString};
use jni::sys::{
    jboolean, jint, jlong, jlongArray, jobjectArray, jsize, jstring,
};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Platform_plus(_: JNIEnv, _: JClass, a: i32, b: i32) -> i32 {
    println!("Loaded from Rust. a = {a}, b = {b}");
    plus(a, b)
}

#[no_mangle]
pub extern "system" fn Java_tokenizers_NativeBridge_newTokenizerFromPretrained(
    mut env: JNIEnv,
    _: JClass,
    identifier: JString,
) -> jlong {
    let id: String = env
        .get_string(&identifier)
        .expect("JNI string conversion failed")
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
pub extern "system" fn Java_tokenizers_NativeBridge_newTokenizerFromFile(
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
pub extern "system" fn Java_tokenizers_NativeBridge_newTokenizerFromBytes(
    mut env: JNIEnv,
    _: JClass,
    bytes: JByteArray,
) -> jlong {
    let len = env
        .get_array_length(&bytes)
        .expect("JNI array length failed.");
    let mut buffer = Vec::new();
    buffer.resize(len as usize, 0);
    match env.get_byte_array_region(&bytes, 0, &mut buffer) {
        Ok(()) => {
            match bridge::new_tokenizer_from_bytes(
                buffer.into_iter().map(|b| b as u8).collect::<Vec<u8>>(),
            ) {
                Ok(ptr) => ptr as jlong,
                Err(err) => {
                    env.throw(err.to_string()).unwrap();
                    0
                }
            }
        }
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_tokenizerEncode(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    input: JString,
    add_special_tokens: jboolean,
) -> jlong {
    let input: String = env
        .get_string(&input)
        .expect("JNI string conversion failed")
        .into();
    match bridge::tokenizer_encode(ptr as usize, input.as_str(), add_special_tokens == 1u8) {
        None => {
            env.throw("Null tokenizer pointer").unwrap();
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
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_tokenizerEncodeBatch(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    inputs: JObjectArray,
    add_special_tokens: bool,
) -> jlongArray {
    let len = env.get_array_length(&inputs).expect("Inputs has no length");
    let inputs: Vec<String> = (0..len)
        .map(|idx| {
            let jstr: JString = env
                .get_object_array_element(&inputs, idx)
                .expect(format!("Failed to index inputs at {idx}").as_str())
                .into();
            let java_str = env.get_string(&jstr).expect("JNI String conversion failed");
            java_str.to_str().unwrap().to_string()
        })
        .collect();

    match bridge::tokenizer_encode_batch(ptr as usize, inputs, add_special_tokens) {
        None => {
            env.throw("Null tokenizer pointer.").unwrap();
            0 as jlongArray
        }
        Some(Ok(pointers)) => {
            let array = env.new_long_array(pointers.len() as jsize).unwrap();
            env.set_long_array_region(
                &array,
                0,
                pointers
                    .into_iter()
                    .map(|p| p as jlong)
                    .collect::<Vec<jlong>>()
                    .as_slice(),
            )
            .unwrap();
            array.into_raw()
        }
        Some(Err(err)) => {
            env.throw(err.to_string()).unwrap();
            0 as jlongArray
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_encodingGetTokenAt(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    index: jint,
) -> jstring {
    match bridge::encoding_get_token_at(&(ptr as usize), index as usize) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0 as jobjectArray
        }
        Some(token) => env.new_string(token).unwrap().into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_encodingGetIdAt(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    index: jint,
) -> jint {
    match bridge::encoding_get_id_at(&(ptr as usize), index as usize) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0
        }
        Some(id) => id as jint,
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_encodingGetSequenceIdAt(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    index: jint,
) -> jlong {
    match bridge::encoding_get_sequence_id_at(&(ptr as usize), index as usize) {
        None => {
            env.throw("Null tokenizer pointer.").unwrap();
            0
        }
        Some(Some(id)) => (id + 1) as jlong,
        Some(None) => 0,
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_encodingGetAttentionMaskAt(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    index: jint,
) -> jint {
    match bridge::encoding_get_attention_mask_at(&(ptr as usize), index as usize) {
        None => {
            env.throw("Null tokenizer pointer.").unwrap();
            0
        }
        Some(id) => id as jint,
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_encodingGetLen(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
) -> jint {
    match bridge::encoding_get_len(&(ptr as usize)) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0
        }
        Some(len) => len as jint,
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_encodingEq(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    other_ptr: jlong,
) -> jboolean {
    match bridge::encoding_eq(&(ptr as usize), &(other_ptr as usize)) {
        None => {
            env.throw("Null encoding pointer.").unwrap();
            0
        }
        Some(eq) => {
            if eq {
                1u8
            } else {
                0u8
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_releaseTokenizer(
    _: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    bridge::release_tokenizer(ptr as usize)
}

#[no_mangle]
pub unsafe extern "system" fn Java_tokenizers_NativeBridge_releaseEncoding(
    _: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    bridge::release_encoding(ptr as usize)
}
