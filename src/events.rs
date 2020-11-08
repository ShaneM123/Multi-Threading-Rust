use std::io::Read;
use reqwest::blocking::get as get_req;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::env;
use lib::Events;

mod lib;



pub fn events() -> Events{
    let predict_secret = env::var("PREDICT_SECRET").unwrap();
    let client = reqwest::blocking::Client::new();
    let mut res = client.get("https://api.predicthq.com/v1/events?location_around.origin=53%2C-6")
        .bearer_auth(predict_secret)
        .send()
        .unwrap()
        .json::<Events>()
        .unwrap();
    return res;
}


