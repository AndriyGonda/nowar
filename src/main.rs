use isahc::{prelude::*, HttpClient};
use std::time::Duration;
mod settings;
mod provider;

fn make_request() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::builder()
        .timeout(Duration::from_secs(5))
        .proxy(Some("socks5://127.0.0.1:9050".parse()?))
        .build()?;
    let mut response = client.get("https://lenta.ru")?;
    println!("{:?}", response.text()?);
    Ok(())
}
fn main() {
    for site in provider::load_sites() {
        println!("{:?}", site.page);
    }
    
    for proxy in provider::load_proxies() {
        println!("{:?}", proxy);
    }
    // match make_request() {
    //     Ok(_) => println!("success"),
    //     Err(_) => println!("error")
    // }
    // let proxies = provider::load_proxies_from_url(
    //     "https://raw.githubusercontent.com/opengs/uashieldtargets/v2/proxy.json",
    // )
    // .unwrap();
    // for proxy in proxies {
    //     println!("{:?}", proxy)
    // }
    // for _ in 1..10 {
    //     let agent = useragent::random_agent();
    //     println!("{:?}", agent);
    // }

    // let sites = provider::load_sites_from_url("https://raw.githubusercontent.com/opengs/uashieldtargets/v2/sites.json").unwrap();
    // for site in sites {
    //     println!("{:?}", site);
    // }
}
