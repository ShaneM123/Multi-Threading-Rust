use std::io::Read;
use reqwest::blocking::get as get_req;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

const PREDICT_SECRET: &str = "XhM9SVnEpHrm5PUxgaGFD6MQRbRRmjG6Hql9LH7u";

#[derive(DeserializeOwned, Deserialize, Debug)]
pub struct Events {

}

pub fn events() -> Events{
    let client = reqwest::blocking::Client::new();
    let mut res = client.get("https://api.predicthq.com/v1/events/")
        .bearer_auth(PREDICT_SECRET)
        .send()
        .unwrap()
        .json::<Events>()
        .unwrap();


    return res;
}


