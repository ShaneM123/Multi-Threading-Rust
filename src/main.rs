#[path="events.rs"]
mod events;
#[path="weather.rs"]
mod weather;

mod lib;

use lib::Forecast;

use std::thread;

fn main() {
    println!("Hello, world!");

    // let events_thread = thread::Builder::new()
    //     .name(String::from("Event Thread"))
    //     .spawn(||{events::events()}).expect("Even thread failed to spawn");
    //
    let weather_thread = thread::Builder::new()
        .name(String::from("Event Thread"))
        .spawn(||{weather::forecast()}).expect("Even thread failed to spawn");

    let forecast = weather_thread.join().unwrap();

    println!("Weather {:?}", forecast.weather.details.description);

  //  events_thread.join().expect("failed to join event thread");


}
