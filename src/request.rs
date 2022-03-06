use jni::{
    objects::{JObject, JString},
    sys::jstring,
    JNIEnv,
};

use crate::util;

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Req_req_1get(
    env: JNIEnv,
    obj: JObject,
    url: JString,
) -> jstring {
    let mut _url: String;
    let mut text: String = String::new();

    if !url.is_null() {
        _url = env.get_string(url).unwrap().into();

        text = util::run_async(async { reqwest::get(_url).await.unwrap().text().await.unwrap() });
    }

    env.delete_local_ref(obj).unwrap();
    env.delete_local_ref(*url).unwrap();

    env.new_string(text).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn Java_cn_smilex_req_Req_req_1post(
    env: JNIEnv,
    obj: JObject,
    url: JString,
) -> jstring {
    let mut _url: String;
    let mut text: String = String::new();

    if !url.is_null() {
        _url = env.get_string(url).unwrap().into();

        text = util::run_async(async {
            reqwest::Client::new()
                .post(_url)
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
