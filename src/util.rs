use jni::objects::{JObject, JString, JValue};
use jni::sys::jint;
use jni::JNIEnv;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Version;
use std::collections::HashMap;

pub const JAVA_TYPE_BYTE: &'static str = "B";
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

pub const JAVA_CLASS_RUNTIME_EXCEPTION: &'static str = "Ljava/lang/RuntimeException";

pub const JAVA_CLASS_HTTP_REQUEST: &'static str = "Lcn/smilex/req/HttpRequest;";
pub const JAVA_CLASS_HTTP_RESPONSE: &'static str = "Lcn/smilex/req/HttpResponse;";
pub const JAVA_CLASS_HTTP_BODY_INTERFACE: &'static str = "Lcn/smilex/req/HttpBodyInterface;";
pub const JAVA_CLASS_HTTP_STRING_BODY: &'static str = "Lcn/smilex/req/HttpStringBody;";
pub const JAVA_CLASS_HTTP_BYTE_ARR_BODY: &'static str = "Lcn/smilex/req/HttpByteArrBody;";

///
/// 将JString并将其转换为 String
///
pub fn jstring_to_string(env: &JNIEnv, obj: &JString) -> String {
    env.get_string(*obj).unwrap().into()
}

///
/// 获取JString
///
pub fn get_jstring<'a>(env: &'a JNIEnv, name: &'static str, obj: &'a JObject) -> JString<'a> {
    let tmp1: JValue = env.get_field(*obj, name, JAVA_CLASS_STRING).unwrap();
    tmp1.l().unwrap().into()
}

///
/// 获取JString并将其转换为 String
///
pub fn get_jstring_to_string<'a>(env: &'a JNIEnv, str: &JString<'a>) -> String {
    env.get_string(*str).unwrap().into()
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

            while env
                .call_method(*(&iter), "hasNext", "()Z", &[])
                .unwrap()
                .z()
                .unwrap()
            {
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

pub fn get_request_body<'a>(env: &JNIEnv, obj: &JObject) -> Option<Vec<u8>> {
    let obj = env
        .get_field(*obj, "body", JAVA_CLASS_HTTP_BODY_INTERFACE)
        .unwrap()
        .l()
        .unwrap();

    if obj.is_null() {
        return None;
    }

    if env
        .is_instance_of(obj, JAVA_CLASS_HTTP_STRING_BODY)
        .unwrap()
    {
        let content = jstring_to_string(env, &get_jstring(env, "content", &obj));
        return Some(content.as_bytes().to_vec());
    } else if env
        .is_instance_of(obj, JAVA_CLASS_HTTP_BYTE_ARR_BODY)
        .unwrap()
    {
        let array = env
            .get_field(obj, "content", "[B")
            .unwrap()
            .l()
            .unwrap()
            .into_inner();
        let array_length = env.get_array_length(array).unwrap();

        let mut arr = vec![0; array_length as usize];
        env.get_byte_array_region(array, 0, &mut arr).unwrap();
        return Some(unsafe {
            std::slice::from_raw_parts(arr.as_ptr() as *const u8, arr.len()).to_vec()
        });
    }

    None
}
