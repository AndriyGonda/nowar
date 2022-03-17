mod provider;
mod useragent;

fn main() {
    // let proxies = provider::load_proxies_from_url(
    //     "https://raw.githubusercontent.com/opengs/uashieldtargets/v2/proxy.json",
    // )
    // .unwrap();
    // for proxy in proxies {
    //     println!("{:?}", proxy)
    // }
    for _ in 1..10 {
        let agent = useragent::random_agent();
        println!("{:?}", agent);
    }
   
    // let sites = provider::load_sites_from_url("https://raw.githubusercontent.com/opengs/uashieldtargets/v2/sites.json").unwrap();
    // for site in sites {
    //     println!("{:?}", site);
    // }
}
