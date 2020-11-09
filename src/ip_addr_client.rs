pub mod amazon_aws;
pub mod http_bin;
pub mod if_config;
pub mod prelude;

use std::net::IpAddr;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

pub use amazon_aws::AmazonAws;
pub use http_bin::HttpBin;
pub use if_config::IfConfig;

#[async_trait]
pub trait IpAddrClient {
    fn get_url(&self) -> String;
    async fn fetch(&self, request: &Client) -> Result<IpAddr>;
}
