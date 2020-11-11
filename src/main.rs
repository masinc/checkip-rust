mod arg_parse;
mod fetch_ip_addr;
mod ip_addr_client;

use anyhow::Result;
use fetch_ip_addr::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = arg_parse::get_clap_app();
    let matches = app.get_matches();

    let client = arg_parse::from_ipaddr_client(&matches);

    eprintln!("fetch for {}", client.get_url());
    let ipaddr = fetch_ip_addr(&*client).await?;
    println!("{}", ipaddr);
    Ok(())
}
