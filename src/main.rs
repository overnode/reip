use std::collections::HashMap;

fn main() {
    let ip = get_external_ip().expect("Error getting ip");
    display_ip(&ip);
}

fn get_external_ip() -> Result<String, minreq::Error> {
    let response = minreq::get("http://httpbin.org/ip")
        .send()?
        .json::<HashMap<String, String>>()?;

    let result = response.get("origin").unwrap();
    Ok(result.to_string())
}

fn display_ip(ip: &String) {
    println!("{}", ip);
}
