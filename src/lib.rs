#[macro_use]
extern crate lazy_static;

pub mod request;
pub mod util;

#[cfg(test)]
mod tests {

    #[test]
    fn test_get() {
        let s = crate::util::run_async(async {
            let s = reqwest::get("https://www.baidu.com").await.unwrap();
            let text = s.text().await.unwrap();
            text
        });

        println!("{}", s);
    }

    #[test]
    fn test_redirect() {
        let custom = reqwest::redirect::Policy::custom(|attempt| attempt.stop());
        let client = reqwest::Client::builder().redirect(custom).build().unwrap();

        let resp = crate::util::run_async(async {
            client
                .get("https://v.douyin.com/NNWNdq6/")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        });

        println!("resp: {}", &resp);
    }
}
