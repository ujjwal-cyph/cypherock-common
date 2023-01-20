pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}
pub mod core {
    include!(concat!(env!("OUT_DIR"), "/core.rs"));
}
pub mod error {
    include!(concat!(env!("OUT_DIR"), "/error.rs"));
}
pub mod btc {
    include!(concat!(env!("OUT_DIR"), "/btc.rs"));
}
pub mod get_device_info {
    include!(concat!(env!("OUT_DIR"), "/get_device_info.rs"));
}
// pub mod add_coin {
//     include!(concat!(env!("OUT_DIR"), "/add_coin.rs"));
// }
