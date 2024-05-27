use std::net::IpAddr;
use std::time::Duration;
use futures;
use tokio;
use tokio_ping;
use std;
use std::thread::sleep;
use argparse::{ArgumentParser, List, Store};
use futures::{Future, Stream};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PingResult {
    usec: u128,
    time_string: String,
    timeout: bool,
    ip: String,
}

struct Options {
    count: u64,
    wait: u64,
    args: Vec<String>,
}

fn main() {
    let mut options = Options { count: 4, wait: 1, args: vec!() };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("This a binary ping for result in json");
        ap.refer(&mut options.count)
            .add_option(&["--count", "-c"], Store,
                        "Total ping send");
        ap.refer(&mut options.wait)
            .add_option(&["--wait", "-w"], Store,
                        "Time wait for ping");
        ap.refer(&mut options.args)
            .add_argument("arguments", List,
                          r#"Arguments for command"#);
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }

    if options.args.is_empty() {
        println!("{}", "Arguments is empty");
        std::process::exit(1);
    }

    if options.count < 1 || options.count > 60 {
        println!("options cannot be less than 1 and bigger the 60, actual value {}", options.count);
        std::process::exit(1);
    }

    if options.wait < 1 || options.wait > 60 {
        println!("options cannot be less than 1 and bigger the 60, actual value {}", options.count);
        std::process::exit(1);
    }

    send_ping(options);
    std::process::exit(0);
}

fn send_ping(options: Options) {
    let addr = options.args.first().unwrap().parse().unwrap();
    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(addr).stream()));
    let future = stream.and_then(move |stream| {
        stream.take(options.count).for_each(move |mb_time| {
            match mb_time {
                Some(time) => print_json(time, addr),
                None => print_json_timeout(),
            }
            sleep(Duration::from_secs(options.wait));
            Ok(())
        })
    });

    tokio::run(future.map_err(|err| {
        eprintln!("Error: {}", err)
    }))
}

fn print_json_timeout() {
    let ping_object = PingResult { usec: 0, time_string: String::new(), timeout: true, ip: String::new() };
    let json_as_string = serde_json::to_string(&ping_object);
    match json_as_string {
        Ok(json) => println!("{}", json),
        Err(err) => println!("{}", err)
    }
}

fn print_json(time: Duration, addr: IpAddr) {
    let ping_object = PingResult { usec: time.as_micros(), time_string: std::format!("{:?}", time), timeout: false, ip: addr.to_string() };
    let json_as_string = serde_json::to_string(&ping_object);
    match json_as_string {
        Ok(json) => println!("{}", json),
        Err(err) => println!("{}", err)
    }
}