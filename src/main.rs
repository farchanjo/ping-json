use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;
use futures;
use tokio;
use tokio_ping;

use futures::{Future, Stream};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PingResult {
    usec: u128,
    time_string: String,
    ip: String
}

fn main() {
    let addr = std::env::args().nth(1).unwrap().parse().unwrap();

    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(addr).stream()));
    let future = stream.and_then(move |stream| {
        stream.take(3).for_each(move |mb_time| {
            match mb_time {
                Some(time) => print_json(time, addr),
                None => println!("timeout"),
            }
            sleep(Duration::from_secs(1));
            Ok(())
        })
    });

    tokio::run(future.map_err(|err| {
        eprintln!("Error: {}", err)
    }))
}


fn print_json(time: Duration, addr: IpAddr) {
    let ping_object = PingResult { usec: time.as_micros(), time_string: std::format!("{:?}", time), ip: addr.to_string() };
    let json_as_string = serde_json::to_string(&ping_object);
    match json_as_string {
        Ok(json) => println!("{}", json),
        Err(err) => println!("{}", err)
    }
}