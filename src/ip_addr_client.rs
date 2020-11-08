pub mod amazon_aws;
pub mod http_bin;
pub mod if_config;

use std::net::IpAddr;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait IpAddrClient {
    fn get_url(&self) -> String;
    async fn fetch(&self, request: &Client) -> Result<IpAddr>;
}

pub async fn fetch_ip_addr(get_ip_addr_client: &impl IpAddrClient) -> Result<IpAddr> {
    let request = reqwest::Client::new();
    Ok(get_ip_addr_client.fetch(&request).await?)
}
