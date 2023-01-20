use crate::proto;
use crate::utils::*;

pub fn create_query() -> proto::core::Query {
    let mut query = proto::core::Query::default();
    let mut btc_query = proto::btc::Request::default();
    let mut add_account_query = proto::btc::AddAccountRequest::default();
    let mut derivation_path = proto::common::DerivationPath::default();
    derivation_path.path = vec![44, 0, 0, 0, 0];

    add_account_query.wallet_id = String::from("DEMO");
    add_account_query.coin_id = String::from("btc");
    add_account_query.derivation_path = Some(derivation_path);
    btc_query.request = Some(proto::btc::request::Request::AddAccount(add_account_query));
    query.request = Some(proto::core::query::Request::Btc(btc_query));

    query
}

pub fn parse_query(query: &proto::core::Query) {
    println!("Parsing query...");
    match &query.request {
        None => println!("None cmd"),
        Some(proto::core::query::Request::Btc(cmd)) => {
            println!("Is Btc Query");

            match &cmd.request {
                None => println!("None cmd"),
                Some(proto::btc::request::Request::AddAccount(cmd)) => {
                    println!("Is Btc Add Account Query");
                    println!("Wallet Id: {}", cmd.wallet_id);
                    println!("Coin Id: {}", cmd.coin_id);

                    match &cmd.derivation_path {
                        None => println!("No derivation path"),
                        Some(path) => {
                            for p in path.path.iter() {
                                print!("{}/", p);
                            }
                            println!();
                        }
                    }
                },
                _ => println!("Unsupported query")
            }

            // println!("Dummy {}", cmd.dummy);
        },
        _ => println!("Unsupported query")
    };
}

pub fn create_result() -> proto::core::Result {
    let mut result = proto::core::Result::default();
    let mut btc_response = proto::btc::Response::default();
    let mut add_account_response = proto::btc::AddAccountResponse::default();
    add_account_response.xpub = String::from("xpub1726389owqyd98127ehw98");

    btc_response.response = Some(proto::btc::response::Response::AddAccount(add_account_response));

    result.response = Some(proto::core::result::Response::Btc(btc_response));
    result
}

pub fn parse_result(result: &proto::core::Result) {
    println!("Parsing result...");
    match &result.response {
        None => println!("None cmd"),
        Some(proto::core::result::Response::Btc(cmd)) => {
            println!("Is Btc Result");

            match &cmd.response {
                None => println!("None cmd"),
                Some(proto::btc::response::Response::AddAccount(cmd)) => {
                    println!("Is Btc Add Account response");

                    println!("Xpub: {}", cmd.xpub);
                },
                _ => println!("Unsupported query")
            }
        },
        _ => println!("Unsupported result")
    };
}

pub fn run() {
    println!("");
    println!("********* BTC Add Account: Started ************");
    let query = create_query();
    let serialized = serialize(&query);
    let deserialized = deserialize_query(&serialized).expect("Error");
    println!("Serialized Query: {:?}", serialized);
    parse_query(&deserialized);

    println!();

    let result = create_result();
    let serialized = serialize(&result);
    let deserialized = deserialize_result(&serialized).expect("Error");
    println!("Serialized Result: {:?}", serialized);
    parse_result(&deserialized);
    println!("********* BTC Add Account: Completed ************");
}
