use std::io::Read;
use reqwest::blocking::get as get_req;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::env;

//const PREDICT_SECRET: &str = "XhM9SVnEpHrm5PUxgaGFD6MQRbRRmjG6Hql9LH7u";

#[derive(Deserialize, Debug)]
pub struct Events {
    count: i32,
    overflow: bool,
    next: String,
    previous: String,
    results: HashMap<String,String>,

}

pub fn events() -> Events{
    let predict_secret = env::var("PREDICT_SECRET").unwrap();
    let client = reqwest::blocking::Client::new();
    let mut res = client.get("https://api.predicthq.com/v1/events?country=IE&limit=2")
        .bearer_auth(predict_secret)
        .send()
        .unwrap()
        .json::<Events>()
        .unwrap();


    return res;
}


