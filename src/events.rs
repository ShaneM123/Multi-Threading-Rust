use std::io::Read;
use reqwest::blocking::get as get_req;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::env;
use lib::Events;

mod lib;

pub fn events( api: String ) -> Events{

    let client = reqwest::blocking::Client::new();
    let mut res = client.get("https://api.predicthq.com/v1/events?location_around.origin=51.507%2C.127")
        .bearer_auth(api)
        .send()
        .unwrap()
        .json::<Events>()
        .unwrap();
    return res;
}


