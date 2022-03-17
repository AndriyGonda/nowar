use isahc::prelude::*;
pub mod proxy;
pub mod site;
use proxy::Proxy;
use site::Site;


pub fn load_proxies_from_url(url: &str) -> Result<Vec<Proxy>, Box<dyn std::error::Error>> {
    let mut response = isahc::get(url)?;
    let content = response.text()?;
    let proxies: Vec<Proxy> = serde_json::from_str(content.as_str())?;
    Ok(proxies)
}

pub fn load_sites_from_url(url: &str) -> Result<Vec<Site>, Box<dyn std::error::Error>> {
    let mut response = isahc::get(url)?;
    let content = response.text()?;
    let sites: Vec<Site> = serde_json::from_str(content.as_str())?;
    Ok(sites)
}