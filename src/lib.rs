pub mod request;

#[cfg(test)]
mod tests {

    #[test]
    fn test_get() {
        let s = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let s = reqwest::get("https://www.baidu.com").await.unwrap();
                let text = s.text().await.unwrap();
                text
            });

        println!("{}", s);
    }
}
