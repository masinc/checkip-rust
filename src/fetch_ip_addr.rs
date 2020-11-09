use std::net::IpAddr;

use anyhow::Result;
use rand::prelude::*;

use crate::ip_addr_client::prelude::*;

pub async fn fetch_ip_addr(ip_addr_client: &(impl IpAddrClient + ?Sized)) -> Result<IpAddr> {
    let request = reqwest::Client::new();
    Ok(ip_addr_client.fetch(&request).await?)
}

pub fn random_client_http() -> Box<dyn IpAddrClient> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0, 2);

    match n {
        0 => Box::new(AmazonAws::new_http()),
        1 => Box::new(HttpBin::new_http()),
        2 => Box::new(IfConfig::new_http()),
        _ => unreachable!(),
    }
}

pub fn random_client_https() -> Box<dyn IpAddrClient> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0, 2);

    match n {
        0 => Box::new(AmazonAws::new_https()),
        1 => Box::new(HttpBin::new_https()),
        2 => Box::new(IfConfig::new_https()),
        _ => unreachable!(),
    }
}

pub fn random_client() -> Box<dyn IpAddrClient> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0, 5);

    match n {
        0 => Box::new(AmazonAws::new_http()),
        1 => Box::new(HttpBin::new_http()),
        2 => Box::new(IfConfig::new_http()),
        3 => Box::new(AmazonAws::new_https()),
        4 => Box::new(HttpBin::new_https()),
        5 => Box::new(IfConfig::new_https()),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[tokio::test]
    async fn test_fetch() {
        struct TestClient;
        #[async_trait]
        impl IpAddrClient for TestClient {
            fn get_url(&self) -> String {
                "localhost".into()
            }

            async fn fetch(&self, _client: &reqwest::Client) -> Result<IpAddr> {
                Ok("127.0.0.1".parse()?)
            }
        }

        let ip_addr_client = TestClient {};

        assert_eq!("localhost", ip_addr_client.get_url());
        assert!(fetch_ip_addr(&ip_addr_client).await.is_ok());
    }
}
