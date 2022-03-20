pub mod proxy;
pub mod site;

use crate::settings;

use cached::proc_macro::cached;
use isahc::prelude::*;

use proxy::Proxy;
use site::Site;

fn load_proxies_from_url(url: &str) -> Result<Vec<Proxy>, Box<dyn std::error::Error>> {
    let mut response = isahc::get(url)?;
    let content = response.text()?;
    let proxies: Vec<Proxy> = serde_json::from_str(content.as_str())?;
    Ok(proxies)
}

fn load_sites_from_url(url: &str) -> Result<Vec<Site>, Box<dyn std::error::Error>> {
    let mut response = isahc::get(url)?;
    let content = response.text()?;
    let sites: Vec<Site> = serde_json::from_str(content.as_str())?;
    Ok(sites)
}

#[cached(time = 600, time_refresh = true)]
pub fn load_sites() -> Vec<Site> {
    let mut sites: Vec<Site> = vec![];
    for origin in settings::SITES_ORIGINS {
        match load_sites_from_url(origin) {
            Ok(loaded) => sites.extend(loaded),
            Err(_) => println!("Unable to load sites from origin {:?}", origin),
        }
    }
    sites
}

#[cached(time = 600, time_refresh = true)]
pub fn load_proxies() -> Vec<Proxy> {
    let mut proxies: Vec<Proxy> = vec![];
    for origin in settings::PROXY_LIST {
        match load_proxies_from_url(origin) {
            Ok(loaded) => proxies.extend(loaded),
            Err(_) => println!("Unable to load proxies from origin {:?}", origin),
        }
    }
    proxies
}

pub fn get_target_sites() -> Vec<Site> {
    let sites = load_sites();
    if sites.is_empty() {
        let default_sites = settings::DEFAULT_TARGETS
            .iter()
            .map(|item| Site {
                page: item.to_string(),
            })
            .collect::<Vec<Site>>();
        return default_sites;
    }
    sites
}
