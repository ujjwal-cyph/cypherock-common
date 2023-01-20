mod proto;
mod flows;
mod utils;

fn main() {
    utils::run_status();
    flows::device_info::run();
    flows::btc::run();
}
