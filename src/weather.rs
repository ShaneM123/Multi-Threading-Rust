use lib::Forecast;

mod lib;

pub fn forecast(api: String) -> Forecast {
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}","Dublin", "IE", api);
    let client = reqwest::blocking::Client::new();
    let  res = client.get(&url)
        .send()
        .unwrap()
        .json::<Forecast>()
        .unwrap();

    return res;
}
