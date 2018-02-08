
extern crate iota_api_rust;
extern crate serde_json;

use serde_json::Value;
use iota_api_rust::build_request_body;

#[test]
fn test_basic() {
    let result = build_request_body("getNodeInfo", 0u32);
    let _value: Value = result.unwrap().json().unwrap();
    //println!("{:#?}", value);
    //println!("{:?}", value["latestMilestone"].as_str().unwrap());
    //println!("{:?}", value["latestSolidSubtangleMilestone"].as_str().unwrap());
}