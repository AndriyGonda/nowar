use crate::provider::proxy::Proxy;
use crate::settings;
use isahc::{prelude::*, HttpClient, HttpClientBuilder};
use std::time::Duration;

pub fn config_client_builder(
    proxy: Option<Proxy>,
) -> Result<HttpClientBuilder, Box<dyn std::error::Error>> {
    let mut client_buider =
        HttpClient::builder().timeout(Duration::from_secs(settings::READ_TIMEOUT_SECONDS));

    client_buider = match proxy {
        Some(proxy_value) => {
            let proxied_builder = match proxy_value.auth {
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
