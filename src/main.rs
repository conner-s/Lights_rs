use std::time::Duration;
use std::net::Ipv4Addr;

mod bulb_scanner;
mod wifi_led_bulb;

fn main() {
    let bulb: Ipv4Addr = bulb_scanner::scan(Duration::from_secs(6))
        .expect("Socket timeout");
    println!("{}", bulb.to_string());

}


