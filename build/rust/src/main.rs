mod proto;

use prost::Message;

pub fn serialize(item: &impl prost::Message) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(item.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    item.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_status(buf: &[u8]) -> Result<proto::common::Status, prost::DecodeError> {
    return proto::common::Status::decode(buf);
}

pub fn deserialize_query(buf: &[u8]) -> Result<proto::common::Query, prost::DecodeError> {
    return proto::common::Query::decode(buf);
}

pub fn deserialize_result(buf: &[u8]) -> Result<proto::common::Result, prost::DecodeError> {
    return proto::common::Result::decode(buf);
}

pub fn create_status(_color: String) -> proto::common::Status {
    let mut status = proto::common::Status::default();
    status.set_cmd_state(proto::common::CmdState::Done);
    status.set_device_waiting_on(proto::common::DeviceWaitingOn::BusyIpCard);
    status
}

pub fn create_query() -> proto::common::Query {
    let mut command = proto::common::Query::default();
    let mut get_device_info = proto::get_device_info::CmdRequest::default();
    get_device_info.dummy = true;
    command.request = Some(proto::common::query::Request::GetDeviceInfo(get_device_info));
    command
}

pub fn parse_query(command: &proto::common::Query) {
    match &command.request {
        None => println!("None cmd"),
        Some(proto::common::query::Request::GetDeviceInfo(cmd)) => {
            println!("Is GetDeviceInfoCmd");
            println!("Dummy {}", cmd.dummy);
        }
    };
}

pub fn create_result() -> proto::common::Result {
    let mut result = proto::common::Result::default();
    let mut get_device_info = proto::get_device_info::CmdResponse::default();

    let mut coin_item_btc = proto::get_device_info::SupportedCoinItem::default();

    coin_item_btc.id = String::from("1");
    coin_item_btc.version = String::from("1.0.0");

    let mut coin_item_eth = proto::get_device_info::SupportedCoinItem::default();

    coin_item_eth.id = String::from("2");
    coin_item_eth.version = String::from("1.0.0");

    get_device_info.device_serial = String::from("9128319287");
    get_device_info.firmware_version = String::from("1.0.2");
    get_device_info.is_authenticated = true;
    get_device_info.coin_list = vec![coin_item_btc, coin_item_eth];

    result.response = Some(proto::common::result::Response::GetDeviceInfo(get_device_info));
    result
}

pub fn parse_result(command: &proto::common::Result) {
    match &command.response {
        None => println!("None cmd"),
        Some(proto::common::result::Response::GetDeviceInfo(cmd)) => {
            println!("Is GetDeviceInfoCmd");
            println!("Device Serial: {}", cmd.device_serial);
            println!("Firmware version: {}", cmd.firmware_version);
            println!("Is Authenticated: {}", cmd.is_authenticated);

            println!("Has Coins: {}", cmd.coin_list.len());

            for coin in cmd.coin_list.iter() {
                println!("\tId: {}", coin.id);
                println!("\tVersion: {}", coin.version);
            }
        }
    };
}

fn main() {
    let mut arg = String::new();
    arg.push_str("asd");
    let status = create_status(arg);
    let serialized = serialize(&status);
    let deserialized = deserialize_status(&serialized).expect("Error");
    println!("Serialized Status: {:?}", serialized);
    println!("CmdState: {}", deserialized.cmd_state);

    let query = create_query();
    let serialized = serialize(&query);
    let deserialized = deserialize_query(&serialized).expect("Error");
    println!("Serialized Query: {:?}", serialized);
    parse_query(&deserialized);

    let result = create_result();
    let serialized = serialize(&result);
    let deserialized = deserialize_result(&serialized).expect("Error");
    println!("Serialized Result: {:?}", serialized);
    parse_result(&deserialized);
}
