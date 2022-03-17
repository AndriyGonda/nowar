// use log::info;
// use log::warn;
// use threadpool::ThreadPool;
// use env_logger::Env;
// use std::sync::{Barrier, Arc};
mod provider;
// fn fetch(url: &str) -> Result<u16, isahc::Error> {
//     let response = isahc::get(url)?;
//     Ok(response.status().as_u16())
// }
use provider::Provider;
fn main() {
    let proxies = Provider::load_proxies_from_url(
        "https://raw1.githubusercontent.com/opengs/uashieldtargets/v2/proxy.json",
    )
    .unwrap();
    for proxy in proxies {
        println!("{:?}", proxy)
    }
    // let proxy = Proxy{ip: "127.0.0.1".to_string(), scheme: "http".to_string()};
    // let serialized = serde_json::to_string(&proxy).unwrap();
    // println!("serialized = {}", serialized);
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // let workers_count = 500;
    // let pool = ThreadPool::new(workers_count);
    // let n_jobs = 100;
    // let barrier = Arc::new(Barrier::new(n_jobs + 1));
    // // let an_atomic = Arc::new(AtomicUsize::new(0));

    // for _ in 0..n_jobs {
    //     let barrier = barrier.clone();
    //     pool.execute(move|| {
    //         match fetch("https://lenta.ru") {
    //             Ok(code) => info!("Site responses with code {}", code),
    //             Err(_) => warn!("Unable to make request to site.")
    //         }

    //         barrier.wait();
    //     });
    // }

    // // barrier.wait();
    // pool.join();// env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // let workers_count = 500;
    // let pool = ThreadPool::new(workers_count);
    // let n_jobs = 100;
    // let barrier = Arc::new(Barrier::new(n_jobs + 1));
    // // let an_atomic = Arc::new(AtomicUsize::new(0));

    // for _ in 0..n_jobs {
    //     let barrier = barrier.clone();
    //     pool.execute(move|| {
    //         match fetch("https://lenta.ru") {
    //             Ok(code) => info!("Site responses with code {}", code),
    //             Err(_) => warn!("Unable to make request to site.")
    //         }

    //         barrier.wait();
    //     });
    // }

    // // barrier.wait();
    // pool.join();
}
