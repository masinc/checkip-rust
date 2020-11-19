#[cfg(test)]
use mockall::{mock, predicate::*};

use std::net::IpAddr;

use super::IpAddrClient;

use async_trait::async_trait;

use anyhow::Result;
use reqwest::Client;

pub struct IfConfig {
    url: &'static str,
}

const URL_HTTP: &str = "http://ifconfig.me";
const URL_HTTPS: &str = "https://ifconfig.me";

impl IfConfig {
    pub fn new_http() -> Self {
        IfConfig { url: URL_HTTP }
    }

    pub fn new_https() -> Self {
        IfConfig { url: URL_HTTPS }
    }
}

// #[cfg_attr(test, automock)]
#[async_trait]
impl IpAddrClient for IfConfig {
    async fn fetch(&self, request: &Client) -> Result<IpAddr> {
        let resp = request.get(self.url).send().await?.text().await?;

        Ok(resp.trim().parse()?)
    }

    fn get_url(&self) -> String {
        self.url.into()
    }
}

#[cfg(test)]
mock! {
    pub IfConfig {
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
    fn test_if_config_http_url() {
        let client = IfConfig::new_http();
        assert_eq!(URL_HTTP, client.get_url());
    }

    #[tokio::test]
    async fn test_if_config_http_fetch() {
        let ctx = MockIfConfig::new_http_context();
        ctx.expect().return_once(|| MockIfConfig::new());
        let mut client = MockIfConfig::new_http();
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
    fn test_if_config_https_url() {
        let client = IfConfig::new_https();
        assert_eq!(URL_HTTPS, client.get_url());
    }
    #[tokio::test]
    async fn test_if_config_https_fetch() {
        let ctx = MockIfConfig::new_https_context();
        ctx.expect().return_once(|| MockIfConfig::new());
        let mut client = MockIfConfig::new_https();
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
