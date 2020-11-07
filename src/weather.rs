use std::io::Read;
use reqwest::blocking::get as get_req;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::env;
use lib::Forecast;

mod lib;

const PREDICT_SECRET: &str = "XhM9SVnEpHrm5PUxgaGFD6MQRbRRmjG6Hql9LH7u";



pub fn forecast() -> Forecast {
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}","IE", "Dublin", env::var("APP_ID").unwrap());
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(&url)
       // .bearer_auth(PREDICT_SECRET)
        .send()
        .unwrap()
        .json::<Forecast>()
        .unwrap();

    return res;
}
