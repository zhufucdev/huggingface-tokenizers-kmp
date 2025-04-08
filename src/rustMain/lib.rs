use jni::JNIEnv;
use jni::objects::JClass;
pub use plus::*;

pub mod plus {
    #[no_mangle]
    pub extern "C" fn plus(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[no_mangle]
pub extern "system" fn Java_Platform_plus(env: JNIEnv, class: JClass, a: i32, b: i32) -> i32 {
    println!("Loaded from Rust. a = {a}, b = {b}");
    plus::plus(a, b)
}

#[cfg(test)]
mod tests {
    use crate::plus;

    #[test]
    fn plus_test() {
        assert_eq!(0, plus(0, 0));
        assert_eq!(0, plus(-1, 1));
        assert_eq!(0, plus(1, -1));
        assert_eq!(4, plus(2, 2));
        assert_eq!(5, plus(3, 2));
        assert_eq!(5, plus(2, 3));
    }
}
