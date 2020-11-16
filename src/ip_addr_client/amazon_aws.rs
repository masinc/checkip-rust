#[cfg(test)]
use mockall::{automock, predicate::*};

use std::net::IpAddr;

use super::IpAddrClient;

use async_trait::async_trait;

use anyhow::Result;
use reqwest::Client;

#[allow(dead_code)]
pub struct AmazonAws {
    url: &'static str,
}

const URL_HTTP: &str = "http://checkip.amazonaws.com/";
const URL_HTTPS: &str = "https://checkip.amazonaws.com/";

impl AmazonAws {
    pub fn new_http() -> Self {
        AmazonAws { url: URL_HTTP }
    }

    pub fn new_https() -> Self {
        AmazonAws { url: URL_HTTPS }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
impl IpAddrClient for AmazonAws {
    fn get_url(&self) -> String {
        self.url.into()
    }

    async fn fetch(&self, request: &Client) -> Result<IpAddr> {
        let resp = request.get(self.url).send().await?.text().await?;

        Ok(resp.trim().parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amazon_aws_http_url() {
        let client = AmazonAws::new_http();

        assert_eq!(URL_HTTP, client.get_url());
    }

    #[tokio::test]
    async fn test_amazon_aws_http_fetch() {
        let request = reqwest::Client::new();
        let mut client = MockAmazonAws::new();
        client
            .expect_fetch()
            .returning(|_| Ok("127.0.0.1".parse()?));

        assert_eq!(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            client.fetch(&request).await.unwrap()
        );
    }

    #[test]
    fn test_amazon_aws_https_url() {
        let client = AmazonAws::new_https();

        assert_eq!(URL_HTTPS, client.get_url());
    }
    #[tokio::test]
    async fn test_amazon_aws_https_fetch() {
        let request = reqwest::Client::new();
        let client = AmazonAws::new_https();

        assert!(client.fetch(&request).await.is_ok());
    }
}
