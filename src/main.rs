use isahc::{prelude::*, HttpClient};
use std::time::Duration;
mod provider;
mod settings;
mod client;
use provider::proxy::Proxy;
use provider::site::Site;
use client::config_client_builder;
fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::builder()
        .timeout(Duration::from_secs(5))
        .proxy(Some("socks5://127.0.0.1:9050".parse()?))
        .build()?;
    let mut response = client.get("https://lenta.ru")?;
    println!("{:?}", response.text()?);
    Ok(())
}

fn fetch(site: Site, proxy: Option<Proxy>) -> Result<(), Box<dyn std::error::Error>> {
    let client_builder = config_client_builder(proxy)?;
    let http_client = client_builder.build()?;
    println!("{:?}", http_client);
    Ok(())
}
fn main() {
     fetch(
         Site {
             page: "test".to_string(),
         },
         None,
     ).unwrap();
}
