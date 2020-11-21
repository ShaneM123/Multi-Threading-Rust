use std::io::Read;
use reqwest::blocking::get as get_req;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::env;
use lib::Forecast;

mod lib;

pub fn forecast(api: String) -> Forecast {
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}","Dublin", "IE", api);
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(&url)
        .send()
        .unwrap()
        .json::<Forecast>()
        .unwrap();

    return res;
}
