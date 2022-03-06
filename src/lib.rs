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
}
