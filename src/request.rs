use jni::objects::{JClass, JValue};
use jni::sys::jobject;
use jni::{
    objects::{JObject, JString},
    sys::jstring,
    JNIEnv,
};

use crate::util;

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests__1request(
    env: JNIEnv,
    obj: JObject,
    http_request: JObject,
) -> jobject {
    let http_request_class: JClass = env.get_object_class(http_request).unwrap();

    let url = util::get_jstring_to_string(&env, "url", &http_request);
    let method = util::get_jint_to_i32(&env, "method", &http_request);

    let headers = util::parse_hash_map(
        &env,
        &env.get_field(http_request, "headers", util::JAVA_CLASS_HASH_MAP)
            .unwrap()
            .l()
            .unwrap(),
    );

    let client_builder = reqwest::ClientBuilder::new().cookie_store(true);

    if let Some(v) = headers {
        // 处理请求头
        println!("{:?}", v);
    }

    let client = client_builder.build().unwrap();

    let resp = util::run_async(async {
        match method {
            0 => client.get(url).send().await.unwrap().text().await.unwrap(),
            1 => client.post(url).send().await.unwrap().text().await.unwrap(),
            _ => String::new(),
        }
    });

    // println!("{}", resp);
    let resp_obj = util::new_response_object(&env);

    env.set_field(
        resp_obj,
        "body",
        util::JAVA_CLASS_STRING,
        JValue::from(JObject::from(env.new_string(resp).unwrap().into_inner())),
    )
    .unwrap();

    env.delete_local_ref(*http_request_class).unwrap();
    env.delete_local_ref(obj).unwrap();

    resp_obj.into_inner()
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
