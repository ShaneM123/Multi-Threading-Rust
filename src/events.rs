use error_chain::error_chain;
use std::io::Read;
use reqwest::blocking::get as get_req;

 const PREDICT_SECRET = "XhM9SVnEpHrm5PUxgaGFD6MQRbRRmjG6Hql9LH7u";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn events()-> Result<String, E>  {
    authenticate();
    let client = reqwest::blocking::Client::new();
    let mut res = client.post("https://api.predicthq.com/v1/events/")
        .bearer_auth(PREDICT_SECRET)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    Ok(body)
}

pub fn authenticate(){

    let auth = get_req("GET /v3/users/me HTTP 1.1
Host: www.eventbriteapi.com
Authorization: Bearer ON35GYCZ35WUAG7SPZPF");

    // sync post request of some json.
// requires feature:
// `ureq = { version = "*", features = ["json"] }`
    let resp = ureq::post("https://www.eventbriteapi.com/v3")
        .set("Authorization", "ON35GYCZ35WUAG7SPZPF")
        .send_json(serde_json::json!({
        "name": "martin",
        "rust": true
    }));

// .ok() tells if response is 200-299.
    if resp.ok() {
        println!("success: {}", resp.into_string()?);
    } else {
        // This can include errors like failure to parse URL or
        // connect timeout. They are treated as synthetic
        // HTTP-level error statuses.
        println!("error {}: {}", resp.status(), resp.into_string()?);
    }
}