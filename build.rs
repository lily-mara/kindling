use chrono::{TimeZone, Utc};
use std::env;

fn main() {
    let now = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap(),
        Err(_) => Utc::now(),
    };

    println!("cargo:rustc-env=BUILD_DATE={}", now);
}
