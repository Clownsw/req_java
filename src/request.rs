use crate::util::{self};
use jni::objects::{JClass, JValue};
use jni::sys::{jboolean, jobject, JNI_TRUE};
use jni::{
    objects::{JObject, JString},
    sys::jstring,
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests_init(env: JNIEnv, class: JClass) {
    util::init(&env);
    env.delete_local_ref(*class).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests__1fast_1request(
    env: JNIEnv,
    class: JClass,
    url: JString,
    is_post: jboolean,
) -> jstring {
    let _url = util::jstring_to_string(&env, &url);
    let _is_post = if is_post == JNI_TRUE { true } else { false };

    let resp_text = util::fast_request(_url, _is_post);

    env.delete_local_ref(*class).unwrap();
    env.delete_local_ref(*url).unwrap();

    env.new_string(resp_text).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests__1request(
    env: JNIEnv,
    class: JClass,
    http_request: JObject,
) -> jobject {
    let http_request_class: JClass = env.get_object_class(http_request).unwrap();

    let url = util::get_jstring_to_string(&env, "url", &http_request);
    println!("url = {}", url);

    let method = util::get_jint_to_i32(&env, "method", &http_request);
    println!("method = {}", method);

    let headers = util::parse_hash_map(
        &env,
        &env.get_field(http_request, "headers", util::JAVA_CLASS_HASH_MAP)
            .unwrap()
            .l()
            .unwrap(),
    );

    let mut client_builder = reqwest::ClientBuilder::new().cookie_store(true);

    // 处理请求头
    if let Some(v) = headers {
        client_builder = client_builder.default_headers(v);
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
    env.delete_local_ref(*class).unwrap();

    resp_obj.into_inner()
}

// #[no_mangle]
// pub extern "system" fn Java_cn_smilex_req_Requests_req_1get(
//     env: JNIEnv,
//     obj: JObject,
//     url: JString,
// ) -> jstring {
//     let mut _url: String;
//     let mut text: String = String::new();

//     if !url.is_null() {
//         _url = env.get_string(url).unwrap().into();

//         text = util::run_async(async {
//             let custom = reqwest::redirect::Policy::custom(|attempt| attempt.stop());

//             reqwest::ClientBuilder::new()
//                 .redirect(custom)
//                 .build()
//                 .unwrap()
//                 .get(_url)
//                 .send()
//                 .await
//                 .unwrap()
//                 .text()
//                 .await
//                 .unwrap()
//         });
//     }

//     env.delete_local_ref(obj).unwrap();
//     env.delete_local_ref(*url).unwrap();

//     env.new_string(text).unwrap().into_inner()
// }
