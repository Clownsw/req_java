use jni::objects::{GlobalRef, JObject, JString, JValue};
use jni::sys::jint;
use jni::JNIEnv;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Version;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;

pub const JAVA_TYPE_INT: &'static str = "I";
pub const JAVA_TYPE_LONG: &'static str = "J";
pub const JAVA_TYPE_BOOLEAN: &'static str = "Z";

pub const JAVA_CLASS_OBJECT: &'static str = "Ljava/lang/Object;";
pub const JAVA_CLASS_STRING: &'static str = "Ljava/lang/String;";
pub const JAVA_CLASS_HASH_MAP: &'static str = "Ljava/util/HashMap;";
pub const JAVA_CLASS_IDENTITY_HASH_MAP: &'static str = "Ljava/util/IdentityHashMap;";
pub const JAVA_CLASS_LIST: &'static str = "Ljava/util/List;";
pub const JAVA_CLASS_SET: &'static str = "Ljava/util/Set;";
pub const JAVA_CLASS_ITERATOR: &'static str = "Ljava/util/Iterator;";

pub const JAVA_CLASS_HTTP_REQUEST: &'static str = "Lcn/smilex/req/HttpRequest;";
pub const JAVA_CLASS_HTTP_RESPONSE: &'static str = "Lcn/smilex/req/HttpResponse;";

lazy_static! {
    pub static ref CLASSES: Mutex<HashMap<&'static str, GlobalRef>> = Mutex::new(HashMap::new());
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
    let object_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_OBJECT).unwrap())
        .unwrap();

    let string_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_STRING).unwrap())
        .unwrap();

    let hash_map_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_HASH_MAP).unwrap())
        .unwrap();

    let identity_hash_map_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_IDENTITY_HASH_MAP).unwrap())
        .unwrap();

    let set_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_SET).unwrap())
        .unwrap();

    let iterator_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_ITERATOR).unwrap())
        .unwrap();

    let http_request_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_HTTP_REQUEST).unwrap())
        .unwrap();

    let http_response_class = env
        .new_global_ref(env.find_class(JAVA_CLASS_HTTP_RESPONSE).unwrap())
        .unwrap();

    let mut classes = CLASSES.lock().unwrap();

    classes.insert("Object", object_class);
    classes.insert("String", string_class);
    classes.insert("HashMap", hash_map_class);
    classes.insert("IdentityHashMap", identity_hash_map_class);
    classes.insert("Set", set_class);
    classes.insert("Iterator", iterator_class);
    classes.insert("HttpRequest", http_request_class);
    classes.insert("HttpResponse", http_response_class);
}

///
/// 从CLASSES通过key获取全局引用
///
pub fn get_global_referener(key: &str) -> Option<GlobalRef> {
    match CLASSES.lock().unwrap().get(&key) {
        Some(v) => Some(v.clone()),
        None => None,
    }
}

///
/// 向CLASSES添加全局引用
///
pub fn add_global_referener<'a, O: Into<JObject<'a>>>(
    env: &'a JNIEnv,
    key: &'static str,
    o: O,
) -> GlobalRef {
    let r = env.new_global_ref(o).unwrap();
    let r_c = r.clone();

    CLASSES.lock().unwrap().insert(key, r);
    r_c
}

///
/// 对add_global_referener方法的封装, 使得调用该方法的代码更加简洁
///
pub fn get_global_ref(env: &JNIEnv, key: &'static str, class_name: &'static str) -> GlobalRef {
    match get_global_referener(key) {
        Some(v) => v,
        None => add_global_referener(&env, key, env.find_class(class_name).unwrap()),
    }
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

        if size > 0 {
            // 进行遍历
            let key_set = get_hash_map_key_set(env, map).l().unwrap();

            let iter = env
                .call_method(
                    key_set,
                    "iterator",
                    format!("(){}", JAVA_CLASS_ITERATOR),
                    &[],
                )
                .unwrap()
                .l()
                .unwrap();

            let mut headers = HashMap::new();

            for _ in 0..size {
                let k: JString = env
                    .call_method(*(&iter), "next", format!("(){}", JAVA_CLASS_OBJECT), &[])
                    .unwrap()
                    .l()
                    .unwrap()
                    .into();

                if !k.is_null() {
                    let _k = jstring_to_string(env, &k);

                    let v: JString = env
                        .call_method(
                            *map,
                            "get",
                            format!("({}){}", JAVA_CLASS_OBJECT, JAVA_CLASS_OBJECT),
                            &[JValue::from(env.new_string(_k.as_str()).unwrap())],
                        )
                        .unwrap()
                        .l()
                        .unwrap()
                        .into();

                    let _v = jstring_to_string(env, &v);

                    headers.insert(_k, _v);
                }
            }

            return Some(headers);
        }
    }

    None
}

///
/// HashMap to HeaderMap
///
pub fn hash_map_to_header_map(map: HashMap<String, String>) -> HeaderMap {
    let mut m = HeaderMap::new();

    for item in map.iter() {
        m.insert(
            HeaderName::from_bytes(item.0.as_bytes()).unwrap(),
            HeaderValue::from_str(item.1.as_str()).unwrap(),
        );
    }

    m
}

pub fn version_to_str(version: Version) -> String {
    match version {
        Version::HTTP_09 => "HTTP_0.9".to_string(),
        Version::HTTP_10 => "HTTP_1.0".to_string(),
        Version::HTTP_11 => "HTTP_1.1".to_string(),
        Version::HTTP_2 => "HTTP_2".to_string(),
        Version::HTTP_3 => "HTTP_3".to_string(),
        _ => String::new(),
    }
}

pub fn for_header_map<T>(map: &HeaderMap, f: T)
where
    T: Fn((&HeaderName, &HeaderValue)) -> (),
{
    for item in map.iter() {
        f(item);
    }
}
