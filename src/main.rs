mod ip_addr_client;

use anyhow::Result;
use ip_addr_client::{
    amazon_aws::AmazonAws, fetch_ip_addr, http_bin::HttpBin, if_config::IfConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    // let ipaddr = fetch_ip_addr(&AmazonAws::new_http()).await?;
    // let ipaddr = fetch_ip_addr(&IfConfig::new()).await?;
    let ipaddr = fetch_ip_addr(&HttpBin::new_https()).await?;
    println!("{}", ipaddr);

    Ok(())
}
