use isahc::prelude::*;
pub mod proxy;
use proxy::Proxy;

pub struct Provider {}

impl Provider {
    pub fn load_proxies_from_url(url: &str) -> Result<Vec<Proxy>, Box<dyn std::error::Error>> {
        let mut response = isahc::get(url)?;
        let content = response.text()?;
        let proxies: Vec<Proxy> = serde_json::from_str(content.as_str())?;
        Ok(proxies)
    }
}
