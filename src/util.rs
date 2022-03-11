use jni::objects::{GlobalRef, JObject, JString, JValue};
use jni::sys::jint;
use jni::JNIEnv;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;

pub const JAVA_TYPE_INT: &'static str = "I";
pub const JAVA_CLASS_STRING: &'static str = "Ljava/lang/String;";
pub const JAVA_CLASS_HASH_MAP: &'static str = "Ljava/util/HashMap;";
pub const JAVA_CLASS_SET: &'static str = "Ljava/util/Set;";

pub const JAVA_CLASS_HTTP_RESPONSE: &'static str = "Lcn/smilex/req/HttpResponse;";

lazy_static! {
    pub static ref CLASSES: Mutex<HashMap<String, GlobalRef>> = Mutex::new(HashMap::new());
}

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
/// 初始化
///
pub fn init(env: &JNIEnv) {
    let string_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_STRING).unwrap())
        .unwrap();
    let hash_map_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_HASH_MAP).unwrap())
        .unwrap();
    let set_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_SET).unwrap())
        .unwrap();

    let mut classes = CLASSES.lock().unwrap();
    classes.insert("String".to_string(), string_class);
    classes.insert("HashMap".to_string(), hash_map_class);
    classes.insert("Set".to_string(), set_class);
}

/// 
/// 从CLASSES通过key获取全局引用
/// 
pub fn get_global_referener(key: String) -> GlobalRef {
    CLASSES.lock().unwrap().get(&key).unwrap().clone()
}

///
/// 快速请求(快速是指没有其他附加参数和检查)
///
pub fn fast_request(url: String, is_post: bool) -> String {
    let client = reqwest::Client::new();

    run_async(async {
        if is_post {
            return client.post(url).send().await.unwrap().text().await.unwrap();
        }
        client.get(url).send().await.unwrap().text().await.unwrap()
    })
}

///
/// 将JString并将其转换为 String
///
pub fn jstring_to_string(env: &JNIEnv, obj: &JString) -> String {
    env.get_string(*obj).unwrap().into()
}

///
/// 获取JString并将其转换为 String
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

///
/// 创建一个响应对象
///
pub fn new_response_object<'a>(env: &'a JNIEnv) -> JObject<'a> {
    env.new_object(JAVA_CLASS_HTTP_RESPONSE, "()V", &[])
        .unwrap()
}

///
/// 获取HashMap的size
///
pub fn get_hash_map_size(env: &JNIEnv, map: &JObject) -> i32 {
    env.call_method(*map, "size", "()I", &[])
        .unwrap()
        .i()
        .unwrap()
}

///
/// 调用HashMap的keySet并返回
///
pub fn get_hash_map_key_set<'a>(env: &'a JNIEnv, map: &'a JObject) -> JValue<'a> {
    env.call_method(*map, "keySet", format!("(){}", JAVA_CLASS_SET), &[])
        .unwrap()
}

///
/// 解析HashMap
///
pub fn parse_hash_map(env: &JNIEnv, map: &JObject) -> Option<HashMap<String, String>> {
    if !map.is_null() {
        let size = get_hash_map_size(env, &map);
        println!("headers size = {}", size);
        if size > 0 {
            // 进行遍历
            let _ = get_hash_map_key_set(env, map);
        }
    }
    None
}
