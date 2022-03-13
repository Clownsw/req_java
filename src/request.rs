use crate::util::{self};
use jni::objects::{JClass, JValue};
use jni::sys::{jboolean, jlong, jobject, JNI_TRUE};
use jni::{
    objects::{JObject, JString},
    sys::jstring,
    JNIEnv,
};
use reqwest::StatusCode;

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
    let url = util::get_jstring_to_string(&env, "url", &http_request);

    let method = util::get_jint_to_i32(&env, "method", &http_request);

    let body = util::get_jstring_to_string(&env, "body", &http_request);
    let body_status = body.eq("");

    let headers = util::parse_hash_map(
        &env,
        &env.get_field(http_request, "headers", util::JAVA_CLASS_HASH_MAP)
            .unwrap()
            .l()
            .unwrap(),
    );

    let params = util::parse_hash_map(
        &env,
        &env.get_field(http_request, "params", util::JAVA_CLASS_HASH_MAP)
            .unwrap()
            .l()
            .unwrap(),
    );

    let mut client_builder = reqwest::ClientBuilder::new().cookie_store(true);

    // 处理请求头
    if let Some(v) = headers {
        client_builder = client_builder.default_headers(util::hash_map_to_header_map(v));
    }

    let client = client_builder.build().unwrap();

    let resp_obj = util::new_response_object(&env);

    let mut status_code = StatusCode::from_u16(500).unwrap();
    let mut version = String::new();
    let mut content_length = 0i64;
    let mut remote_address = String::new();
    let mut resp_body = String::new();

    util::run_async(async {
        let req = match method {
            0 => Some(client.get(url)),
            1 => Some(client.post(url)),
            2 => Some(client.put(url)),
            3 => Some(client.delete(url)),
            _ => None,
        };

        if let Some(r) = req {
            let mut r = r;

            // 设置请求参数
            if !body_status {
                r = r.body(body);
            }

            // 设置请求参数
            if let Some(v) = params {
                r = r.query(&v);
            }

            let resp = r.send().await.unwrap();
            status_code = resp.status();
            version = util::version_to_str(resp.version());

            if let Some(v) = resp.content_length() {
                content_length = v as i64;
            }

            if let Some(v) = resp.remote_addr() {
                remote_address = v.to_string();
            }

            // 设置响应头
            util::for_header_map(resp.headers(), |item| {
                let header_name = item.0.as_str();
                let header_value = item.1.to_str().unwrap();

                let headers = env
                    .get_field(resp_obj, "headers", util::JAVA_CLASS_IDENTITY_HASH_MAP)
                    .unwrap()
                    .l()
                    .unwrap();

                env.call_method(
                    headers,
                    "put",
                    format!(
                        "({}{}){}",
                        util::JAVA_CLASS_OBJECT,
                        util::JAVA_CLASS_OBJECT,
                        util::JAVA_CLASS_OBJECT
                    ),
                    &[
                        JValue::from(JObject::from(env.new_string(header_name).unwrap())),
                        JValue::from(JObject::from(env.new_string(header_value).unwrap())),
                    ],
                )
                .unwrap();
            });

            resp_body = resp.text().await.unwrap();
        }
    });

    // 设置响应体
    env.set_field(
        resp_obj,
        "body",
        util::JAVA_CLASS_STRING,
        JValue::from(JObject::from(
            env.new_string(resp_body).unwrap().into_inner(),
        )),
    )
    .unwrap();

    // 设置状态码
    env.set_field(
        resp_obj,
        "statusCode",
        util::JAVA_CLASS_STRING,
        JValue::from(JObject::from(
            env.new_string(status_code.as_str()).unwrap().into_inner(),
        )),
    )
    .unwrap();

    // 设置HTTP版本
    env.set_field(
        resp_obj,
        "version",
        util::JAVA_CLASS_STRING,
        JValue::from(JObject::from(env.new_string(version).unwrap().into_inner())),
    )
    .unwrap();

    // 设置响应正文长度
    env.set_field(
        resp_obj,
        "contentLength",
        util::JAVA_TYPE_LONG,
        JValue::from(jlong::from(content_length)),
    )
    .unwrap();

    // 设置远程地址
    env.set_field(
        resp_obj,
        "remoteAddress",
        util::JAVA_CLASS_STRING,
        JValue::from(JObject::from(env.new_string(remote_address).unwrap())),
    )
    .unwrap();

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
