#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;

use chrono::{Local};
use local_ip_address::local_ip;

use std::time::SystemTime;

#[derive(Serialize)]
struct NumberResponse {
    number: u64,
    is_prime_number: bool,
    execution_time_in_micros: u128
}

#[get("/")]
fn index() -> &'static str {
    "Test example for my rust app!"
}

#[get("/isPrime?<number>")]
fn get_is_prime(number: u64) -> Json<NumberResponse> {
    let now = SystemTime::now();

    Json(NumberResponse {
        number,
        is_prime_number: is_prime(number),
        execution_time_in_micros: now.elapsed().unwrap().as_micros(),
    })
}

#[get("/time")]
fn get_time() -> String {
    let current_time: String = Local::now().time().format("%H:%M:%S").to_string();
    current_time
}

#[get("/ip")]
async fn get_ip() -> String {
    let ip_local: String;
    let public_ip: String;

    match local_ip() {
        Ok(ip) =>   {
            ip_local = ip.to_string();
        }
        Err(e) => {
            println!("{}", e);
            return String::from("There was an error");
        }
    }

    match public_ip::addr_v4().await {
        Some(ip) => {
            public_ip = ip.to_string();
        },
        None => {
            public_ip = String::from("Couldn't find public ip");
        }
    }

    format!("private ip: {} and public ip: {}", ip_local, public_ip)

}

// is prime helper function
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }

    true
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_is_prime, get_time, get_ip])
}
