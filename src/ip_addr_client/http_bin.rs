#[cfg(test)]
use mockall::{mock, predicate::*};

use std::net::IpAddr;

use super::IpAddrClient;

use async_trait::async_trait;

use anyhow::Result;
use reqwest::Client;

pub struct HttpBin {
    url: &'static str,
}

const URL_HTTP: &str = "http://httpbin.org/ip";
const URL_HTTPS: &str = "https://httpbin.org/ip";

impl HttpBin {
    pub fn new_http() -> Self {
        HttpBin { url: URL_HTTP }
    }

    pub fn new_https() -> Self {
        HttpBin { url: URL_HTTPS }
    }
}

// #[cfg_attr(test, automock)]
#[async_trait]
impl IpAddrClient for HttpBin {
    fn get_url(&self) -> String {
        self.url.into()
    }

    async fn fetch(&self, request: &Client) -> Result<IpAddr> {
        #[derive(serde::Deserialize, Debug)]
        struct Response {
            origin: String,
        }

        let resp = request
            .get(self.url)
            .send()
            .await?
            .json::<Response>()
            .await?;

        Ok(resp.origin.parse()?)
    }
}

#[cfg(test)]
mock! {
    pub HttpBin {
        fn new_http() -> Self;
        fn new_https() -> Self;
    }

    #[async_trait]
    trait IpAddrClient {
        fn get_url(&self) -> String;
        async fn fetch(&self, request: &Client) -> Result<IpAddr>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_bin_http_url() {
        let client = HttpBin::new_http();

        assert_eq!(URL_HTTP, client.get_url());
    }

    #[tokio::test]
    async fn test_http_bin_http_fetch() {
        let ctx = MockHttpBin::new_http_context();
        ctx.expect().return_once(|| MockHttpBin::new());
        let mut client = MockHttpBin::new_http();
        client
            .expect_fetch()
            .returning(|_| Ok("127.0.0.1".parse()?));

        let request = reqwest::Client::new();
        assert_eq!(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            client.fetch(&request).await.unwrap()
        );
    }

    #[test]
    fn test_http_bin_https_url() {
        let client = HttpBin::new_https();

        assert_eq!(URL_HTTPS, client.get_url());
    }

    #[tokio::test]
    async fn test_http_bin_https_fetch() {
        let ctx = MockHttpBin::new_https_context();
        ctx.expect().return_once(|| MockHttpBin::new());
        let mut client = MockHttpBin::new_https();
        client
            .expect_fetch()
            .returning(|_| Ok("127.0.0.1".parse()?));

        let request = reqwest::Client::new();
        assert_eq!(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            client.fetch(&request).await.unwrap()
        );
    }
}
