use std::collections::HashMap;

fn main() {
    let mut counter = 0;
    loop {
        // Try 3 times then bail
        if counter >= 3 {
            break;
        }

        // Sleep for 1 second between tries
        if counter > 0 {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        counter += 1;

        match get_external_ip() {
            Ok(ip) => {
                display_ip(&ip);
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }
}

/// Get external IP address
fn get_external_ip() -> Result<String, minreq::Error> {
    let response = minreq::get("http://httpbin.org/ip")
        .with_timeout(5)
        .send()?
        .json::<HashMap<String, String>>()?;

    let result = response.get("origin").unwrap();
    Ok(result.to_string())
}

/// Display IP address
fn display_ip(ip: &str) {
    println!("{}", ip);
}
