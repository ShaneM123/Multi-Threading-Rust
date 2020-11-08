use serde::{Serialize, Deserialize, Deserializer};
use std::iter::Map;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Weather,
    pub base: String,
    pub main: Temps,
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


#[derive(Deserialize, Debug)]
pub struct Events {
    pub count: i32,
    pub overflow: bool,
    pub next: String,
    pub previous: String,
    pub results: Results,
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
    pub aviation_rank: String,
    #[serde(deserialize_with="parse_nulls")]
    pub phq_attendance: String,
    pub entities: Entities,
    #[serde(flatten)]
    pub the_rest: HashMap<String,String>
}

#[derive(Deserialize, Debug, )]
pub struct Entities {
    pub formatted_address: String,
    pub entity_id: String,
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub venue_type: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Element {
//     #[serde(deserialize_with="parse_nulls")]
//     color: String,
// }

fn parse_nulls<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("blank".to_string())
        })
}

//
// {
// "relevance": 0.13139634,
// "id": "Z8aobfiwCjGtkCHVG9",
// "title": "Wicklow Gaol Day Time Tour",
// "description": "",
// "category": "community",
// "labels": [
// "community",
// "family"
// ],
// "rank": 0,
// "local_rank": 0,
// "aviation_rank": null,
// "phq_attendance": null,
// "entities": [
// {
// "formatted_address": "Kilmantin Hill\nWicklow\nIreland",
// "entity_id": "nvD8iVmgnviHp4k94sbkse",
// "name": "Wicklow's Historic Gaol",
// "type": "venue"
// }
// ],
// "duration": 0,
// "start": "2020-12-20T10:30:00Z",
// "end": "2020-12-20T10:30:00Z",
// "updated": "2020-07-06T20:39:23Z",
// "first_seen": "2020-01-04T23:17:54Z",
// "timezone": "Europe/Dublin",
// "location": [
// -6.037282,
// 52.978893
// ],
// "scope": "locality",
// "country": "IE",
// "place_hierarchies": [
// [
// "6295630",
// "6255148",
// "2963597",
// "7521314",
// "2960935",
// "2960936"
// ]
// ],
// "state": "active",
// "brand_safe": true
// }
