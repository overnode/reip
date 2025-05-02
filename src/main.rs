use std::collections::HashMap;

fn main() {
    for iteration in 1..=3 {
        if iteration > 1 {
            eprintln!("Retrying in 1 second...");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        if let Ok(ip) = get_external_ip() {
            display_ip(&ip);
            break;
        } else {
            eprintln!("Error: Failed to retrieve external IP.");
        }
    }
}

/// Get external IP address
fn get_external_ip() -> Result<String, minreq::Error> {
    let response = minreq::get("http://httpbin.org/ip")
        .with_timeout(5) // Set a timeout of 5 seconds for the request
        .send()?
        .json::<HashMap<String, String>>()?;

    let ip = response.get("origin").unwrap_or_else(|| {
        eprintln!("Error: 'origin' key not found in response.");
        std::process::exit(1);
    });
    Result::Ok(ip.to_string())
}

/// Display IP address
fn display_ip(ip: &str) {
    println!("{}", ip);
}
