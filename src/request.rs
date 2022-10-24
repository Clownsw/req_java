use std::sync::{Arc, Mutex};

use crate::util::{self};
use bytes::Bytes;
use jni::objects::{JClass, JValue};
use jni::sys::{jlong, jobject};
use jni::{objects::JObject, JNIEnv};
use reqwest::StatusCode;

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Requests__1request(
    env: JNIEnv,
    class: JClass,
    http_request: JObject,
) -> jobject {
    // 请求URL
    let _url = util::get_jstring(&env, "url", &http_request);
    let url = util::get_jstring_to_string(&env, &_url);

    // 请求方式
    let method = util::get_jint_to_i32(&env, "method", &http_request);

    // 请求体
    let body = util::get_request_body(&env, &http_request);

    // 是否开启获取bytes
    let enable_data_byte = env
        .get_field(http_request, "enableDataByte", util::JAVA_TYPE_BOOLEAN)
        .unwrap()
        .z()
        .unwrap();

    // 请求头
    let headers = util::parse_hash_map(
        &env,
        &env.get_field(http_request, "headers", util::JAVA_CLASS_HASH_MAP)
            .unwrap()
            .l()
            .unwrap(),
    );

    // 查询参数
    let params = util::parse_hash_map(
        &env,
        &env.get_field(http_request, "params", util::JAVA_CLASS_HASH_MAP)
            .unwrap()
            .l()
            .unwrap(),
    );

    // 最大重定向
    let max_redirect: usize = util::get_jint_to_i32(&env, "maxRedirect", &http_request) as usize;

    // 重定向URL列表
    let redirect_url_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let redirect_url_list_clone = redirect_url_list.clone();

    let mut client_builder = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::custom(move |attempt| {
            // 将重定向url保存起来
            (*redirect_url_list_clone.lock().unwrap()).push(attempt.url().to_string());

            // 如果重定向次数大于最大重定向次数则停止
            if attempt.previous().len() > max_redirect {
                return attempt.stop();
            }

            attempt.follow()
        }))
        .cookie_store(true);

    // 处理请求头
    if let Some(v) = headers {
        client_builder = client_builder.default_headers(util::hash_map_to_header_map(v));
    }

    let client = client_builder.build().unwrap();

    // 开始处理响应信息
    let resp_obj = util::new_response_object(&env);

    let mut status_code = StatusCode::from_u16(500).unwrap();
    let mut version = String::new();
    let mut content_length = 0i64;
    let mut remote_address = String::new();
    let mut data_byte: Bytes = Bytes::new();
    let mut resp_body = String::new();

    let req = match method {
        0 => Some(client.get(url)),
        1 => Some(client.post(url)),
        2 => Some(client.put(url)),
        3 => Some(client.delete(url)),
        _ => None,
    };

    if let Some(mut r) = req {
        // 设置请求参数
        if let Some(v) = body {
            r = r.body(v);
        }

        // 设置请求参数
        if let Some(v) = params {
            r = r.query(&v);
        }

        // 发送请求
        let resp = r.send();

        // 如果成功则设置响应数据
        // 否则将错误信息传回body
        if let Ok(resp) = resp {
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

            if enable_data_byte {
                data_byte = resp.bytes().unwrap();
            } else {
                resp_body = resp.text().unwrap();
            }
        } else if let Err(err) = resp {
            resp_body = err.to_string();
        }
    }

    // 释放字符串
    let _url_ptr = env.get_string_utf_chars(_url).unwrap();
    env.release_string_utf_chars(_url, _url_ptr).unwrap();

    // 设置响应体
    if !enable_data_byte {
        env.set_field(
            resp_obj,
            "body",
            util::JAVA_CLASS_STRING,
            JValue::from(JObject::from(
                env.new_string(resp_body).unwrap().into_inner(),
            )),
        )
        .unwrap();
    }

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

    // 设置重定向URL列表
    let redirect_url_lists = env
        .get_field(resp_obj, "redirectUrls", util::JAVA_CLASS_LIST)
        .unwrap()
        .l()
        .unwrap();

    for item in redirect_url_list.lock().unwrap().iter() {
        env.call_method(
            redirect_url_lists,
            "add",
            format!("({}){}", util::JAVA_CLASS_OBJECT, util::JAVA_TYPE_BOOLEAN),
            &[JValue::from(JObject::from(
                env.new_string(item).unwrap().into_inner(),
            ))],
        )
        .unwrap();
    }

    // 设置响应数据
    if enable_data_byte {
        env.set_field(
            resp_obj,
            "dataByte",
            format!("[{}", util::JAVA_TYPE_BYTE),
            JValue::from(JObject::from(
                env.byte_array_from_slice(&data_byte[..]).unwrap(),
            )),
        )
        .unwrap();
    }

    env.delete_local_ref(*class).unwrap();

    resp_obj.into_inner()
}
