#[cfg(test)]
use mockall::{automock, predicate::*};

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

#[cfg_attr(test, automock)]
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
mod tests {
    use super::*;

    #[test]
    fn test_http_bin_http_url() {
        let client = HttpBin::new_http();

        assert_eq!(URL_HTTP, client.get_url());
    }

    #[tokio::test]
    async fn test_http_bin_http_fetch() {
        let request = reqwest::Client::new();
        let mut client = MockHttpBin::new();
        client
            .expect_fetch()
            .returning(|_| Ok("127.0.0.1".parse()?));

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
        let request = reqwest::Client::new();
        let client = HttpBin::new_https();

        assert!(client.fetch(&request).await.is_ok());
    }
}
