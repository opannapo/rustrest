use rustrest::config::config::Config;
use rustrest::debug_info;
use rustrest::util::log as custom_log;

fn main() {
    custom_log::init();
    debug_info!("main api");

    let cfg = Config::new();
    debug_info!("main api {:?}", cfg);
}
