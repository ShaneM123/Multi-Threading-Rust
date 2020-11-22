#[path="events.rs"]
mod events;
#[path="weather.rs"]
mod weather;

mod lib;

use std::thread;

use envfile;
use envfile::EnvFile;
use std::io;
use std::path::Path;


fn main()-> io::Result<()> {
    //https://docs.rs/envfile/0.2.1/envfile/
    let envfile = EnvFile::new(&Path::new(".env"))?;

    let weather_api_secret = envfile.get("APP_ID").expect("failed to unwrap APP_ID").to_owned();
    let event_api_secret = envfile.get("PREDICT_SECRET").expect("failed to unwrap PREDICT_SECRET").to_owned();
    println!("24");
    let events_thread = thread::Builder::new()
        .name(String::from("Event Thread"))
        .spawn(||{events::events(event_api_secret)}).expect("Even thread failed to spawn");
    println!("28");
    let weather_thread = thread::Builder::new()
        .name(String::from("Event Thread"))
        .spawn(||{weather::forecast(weather_api_secret)}).expect("Even thread failed to spawn");

    let forecast = weather_thread.join().unwrap();
    println!("Weather {:?}", forecast.weather.details.description);

    let events = events_thread.join().expect("failed to join event thread");

    let all_events:Vec<&String> = events.results.iter().map(|x| { &x.title }).collect();
    let cloudy_events:Vec<&&String> = all_events.iter().filter(|x| !x.contains("outdoor") && x.len() < 25 ).collect();

    if forecast.weather.details.description.contains("clouds"){
        println!(" here are some Potential event for a cloudy day: {:?}", cloudy_events);
    }
    else {
        println!("here are some potential events:{:?}", all_events);
    }
    Ok(())
}

