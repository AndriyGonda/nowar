mod agent;
use crate::provider::proxy::Proxy;
use crate::provider::site::Site;
use crate::settings;
use isahc::{prelude::*, HttpClient, HttpClientBuilder};
use std::time::Duration;

fn config_client_builder(
    proxy: Option<Proxy>,
) -> Result<HttpClientBuilder, Box<dyn std::error::Error>> {
    let mut client_buider: HttpClientBuilder =
        HttpClient::builder().timeout(Duration::from_secs(settings::READ_TIMEOUT_SECONDS));

    client_buider = match proxy {
        Some(proxy_value) => {
            let proxied_builder: HttpClientBuilder = match proxy_value.auth {
                Some(auth_value) => {
                    let auth_data: Vec<&str> = auth_value.split(":").collect();
                    client_buider
                        .proxy(Some(
                            format!("{:?}://{:?}", proxy_value.scheme, proxy_value.ip).parse()?,
                        ))
                        .proxy_authentication(isahc::auth::Authentication::basic())
                        .proxy_credentials(isahc::auth::Credentials::new(
                            auth_data[0],
                            auth_data[1],
                        ))
                }

                None => client_buider.proxy(Some(
                    format!("{:?}://{:?}", proxy_value.scheme, proxy_value.ip).parse()?,
                )),
            };
            proxied_builder
        }
        None => client_buider,
    };
    Ok(client_buider)
}

pub fn new_client(proxy: Option<Proxy>) -> Result<HttpClient, Box<dyn std::error::Error>> {
    let mut client_builder = config_client_builder(proxy)?;
    client_builder = client_builder.default_headers(&[
        ("Content-Type", "text/html;"),
        ("Connection", "keep-alive"),
        ("Cache-Control", "no-store"),
        ("Accept", "text/*, text/html, text/html;level=1, */*"),
        ("Accept-Language", "ru"),
        ("Accept-Encoding", "gzip, deflate"),
        ("User-Agent", agent::random_agent().as_str()),
    ]);
    let client = client_builder.build()?;
    Ok(client)
}

pub fn fetch(
    site: &Site,
    proxy: Option<Proxy>,
) -> Result<isahc::Response<isahc::Body>, Box<dyn std::error::Error>> {
    let http_client = new_client(proxy)?;
    let response = http_client.get(&site.page)?;
    Ok(response)
}
