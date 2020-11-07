use serde::{Serialize,Deserialize};

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
