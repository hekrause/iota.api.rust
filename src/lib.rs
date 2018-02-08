
extern crate serde_json;
extern crate reqwest;
#[macro_use] extern crate hyper;
#[macro_use] extern crate error_chain;

use serde_json::Value;
use hyper::header::{Headers, ContentType};

const REMOTE_URL: &str = "http://94.130.228.79:14265";

error_chain! {
    foreign_links {
        SerdeError(serde_json::Error);
        ReqwestError(reqwest::Error);
    }
}

pub fn build_request_body(command: &str, depth: u32) -> Result<reqwest::Response>{
    let headers = get_headers();
    let client = reqwest::Client::new();

    let mut raw_command = "".to_string();
    if depth == 0 {
        raw_command = r#"{"command": ""#.to_string() + &command + r#""}"#;
    } else {
        raw_command =   r#"{"command": ""#.to_string() + &command + r#"","# + "\n" +
            r#" "depth": "# + &depth.to_string() + r#"}"#;
    }

    let stringified: Value = serde_json::from_str(&raw_command)?;

    let res = client.post(REMOTE_URL)
        .json(&stringified)
        .headers(headers)
        .send()?;
    Ok(res)
}

fn get_headers() -> hyper::Headers {
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(IOTAAPIVersion("1".to_owned()));
    return headers;
}
/*
fn getNodeInfo() { unimplemented!()}
fn getNeighbors() { unimplemented!()}
fn addNeighbors() { unimplemented!()}
fn removeNeighbors() { unimplemented!()}
fn getTips() { unimplemented!()}
fn findTransactions() { unimplemented!()}
fn getTrytes() { unimplemented!()}
fn getInclusionStates() { unimplemented!()}
fn getBalances() { unimplemented!()}
fn getTransactionToApprove() { unimplemented!()}
fn attachToTangle() { unimplemented!()}
fn interruptAttachingToTangle() { unimplemented!()}
fn broadcastTransactions() { unimplemented!()}
fn storeTransactions() { unimplemented!()}
*/
#[macro_use]
header! { (IOTAAPIVersion, "X-IOTA-API-Version") => [String] }
