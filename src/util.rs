use std::future::Future;
use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use jni::sys::jint;

pub const JAVA_TYPE_INT: &'static str = "I";
pub const JAVA_CLASS_STRING: &'static str = "Ljava/lang/String;";
pub const JAVA_CLASS_HTTP_RESPONSE: &'static str = "Lcn/smilex/req/HttpResponse;";

///
/// 提供异步执行环境
///
pub fn run_async<F: Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f)
}

///
/// 将JString(Java String) 转换为 String
///
pub fn get_jstring_to_string(env: &JNIEnv, name: &'static str, obj: &JObject) -> String {
    let tmp1: JValue = env.get_field(*obj, name, JAVA_CLASS_STRING).unwrap();
    let tmp2: JString = tmp1.l().unwrap().into();
    env.get_string(tmp2).unwrap().into()
}

///
/// 将JValue转为jint
///
pub fn get_jint_to_i32(env: &JNIEnv, name: &'static str, obj: &JObject) -> jint {
    let tmp1: JValue = env.get_field(*obj, name, JAVA_TYPE_INT).unwrap();
    tmp1.i().unwrap()
}