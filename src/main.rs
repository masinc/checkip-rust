mod fetch_ip_addr;
mod ip_addr_client;

use anyhow::Result;
use fetch_ip_addr::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = random_client();
    eprintln!("fetch for {}", client.get_url());
    let ipaddr = fetch_ip_addr(&*client).await?;
    println!("{}", ipaddr);
    Ok(())
}
