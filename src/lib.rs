use serde::{Serialize, Deserialize, Deserializer,};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Weather,
    pub base: String,
    pub main: Temps,
    // flatten the rest
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
   pub details: Details
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Temps {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
   pub all: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub r#type: f64,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

//******EVENTS8*******//

#[derive(Deserialize, Debug)]
pub struct Events {
    pub count: i32,
    pub overflow: bool,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Results>,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    pub relevance: f64,
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub labels: Vec<String>,
    pub rank: i32,
    pub local_rank: i32,
   #[serde(deserialize_with="parse_nulls")]
    pub aviation_rank: i32,
   #[serde(deserialize_with="parse_nulls")]
    pub phq_attendance: i32,
    pub entities: Vec<Entities>,
    #[serde(flatten)]
    _ignore: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, )]
pub struct Entities {
    pub formatted_address: String,
    pub entity_id: String,
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub venue_type: String,
}

fn parse_nulls<'de, D>(d: D) -> Result<i32, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or(0)
        })
}
