#[path="events.rs"]
mod events;
#[path="weather.rs"]
mod weather;

use std::thread;

fn main() {
    println!("Hello, world!");
    // let weather_thread =  thread::Builder::new()
    //     .name(String::from("weather thread"))
    //     .spawn(weather::weather()).expect("weather thread failed to spawn");

    let events_thread = thread::Builder::new()
        .name(String::from("Event Thread"))
        .spawn(events::events()).expect("Even thread failed to spawn");

  //  weather_thread.join().expect("failed to join weather thread");

    events_thread.join().expect("failed to join event thread");

}
