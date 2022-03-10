use jni::{
    JNIEnv,
    objects::{JObject, JString},
    sys::jstring,
};
use jni::objects::{JClass, JValue};
use jni::sys::jobject;

use crate::util;

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests__1request(
    env: JNIEnv,
    obj: JObject,
    http_request: JObject,
) -> jobject {
    let http_request_class: JClass = env.get_object_class(http_request).unwrap();

    let url = util::get_jstring_to_string(&env, "url", &http_request);
    println!("url = {}", url);

    let method = util::get_jint_to_i32(&env, "method", &http_request);
    println!("method = {}", method);

    let client = reqwest::ClientBuilder::new().build().unwrap();

    let resp = util::run_async(async {
        match method {
            0 => {
                client.get(url).send().await.unwrap().text().await.unwrap()
            }
            1 => {
                client.post(url).send().await.unwrap().text().await.unwrap()
            }
            _ => String::new(),
        }
    });

    // println!("{}", resp);

    let http_response_class = env.find_class(util::JAVA_CLASS_HTTP_RESPONSE).unwrap();
    let o = env.new_object(http_response_class, "()V", &[]).unwrap();

    env.set_field(o, "body", util::JAVA_CLASS_STRING, JValue::from(JObject::from(env.new_string(resp).unwrap().into_inner()))).unwrap();

    env.delete_local_ref(*http_request_class).unwrap();
    env.delete_local_ref(obj).unwrap();

    o.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests_req_1get(
    env: JNIEnv,
    obj: JObject,
    url: JString,
) -> jstring {
    let mut _url: String;
    let mut text: String = String::new();

    if !url.is_null() {
        _url = env.get_string(url).unwrap().into();

        text = util::run_async(async {
            let custom = reqwest::redirect::Policy::custom(|attempt| attempt.stop());
            reqwest::ClientBuilder::new()
                .redirect(custom)
                .build()
                .unwrap()
                .get(_url)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        });
    }

    env.delete_local_ref(obj).unwrap();
    env.delete_local_ref(*url).unwrap();

    env.new_string(text).unwrap().into_inner()
}