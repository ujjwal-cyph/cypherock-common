use crate::proto;

use prost::Message;

pub fn serialize(item: &impl prost::Message) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(item.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    item.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_status(buf: &[u8]) -> Result<proto::core::Status, prost::DecodeError> {
    return proto::core::Status::decode(buf);
}

pub fn deserialize_query(buf: &[u8]) -> Result<proto::core::Query, prost::DecodeError> {
    return proto::core::Query::decode(buf);
}

pub fn deserialize_result(buf: &[u8]) -> Result<proto::core::Result, prost::DecodeError> {
    return proto::core::Result::decode(buf);
}

pub fn create_status(_color: String) -> proto::core::Status {
    let mut status = proto::core::Status::default();
    status.set_cmd_state(proto::core::CmdState::Done);
    status.set_device_waiting_on(proto::core::DeviceWaitingOn::BusyIpCard);
    status
}

pub fn run_status() {
    println!("");
    println!("********* Status: Started ************");
    let mut arg = String::new();
    arg.push_str("asd");
    let status = create_status(arg);
    let serialized = serialize(&status);
    let deserialized = deserialize_status(&serialized).expect("Error");
    println!("Serialized Status: {:?}", serialized);
    println!("CmdState: {}", deserialized.cmd_state);
    println!("********* Status: Completed ************");
}
