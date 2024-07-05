use std::env;
use std::env::Args;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::process;

use chrono::prelude::Utc;

use serde::Serialize;

pub struct Config {
    addr: SocketAddr,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Self, Box<dyn std::error::Error>> {
        args.next();

        let ip = match args.next() {
            Some(arg) => arg.parse::<Ipv4Addr>()?,
            None => return Err("IP argument missing".into()),
        };

        let port = match args.next() {
            Some(arg) => arg.parse::<u16>()?,
            None => return Err("Port argument missing".into()),
        };

        if args.next().is_some() {
            return Err("Too many arguments".into());
        };

        let addr = SocketAddr::new(IpAddr::V4(ip), port);

        Ok(Self { addr })
    }
}

#[derive(Serialize)]
struct ReturnValue {
    ip: IpAddr,
    time: String,
}

#[allow(clippy::needless_pass_by_value)]
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(config.addr)?;

    let mut buf = [0; 0];

    loop {
        let (_, addr) = socket.recv_from(&mut buf)?;

        let return_value = serde_json::to_string(&ReturnValue {
            ip: addr.ip(),
            time: Utc::now().to_string(),
        })?;

        socket.send_to(return_value.as_bytes(), addr)?;
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Setting up application failed: {err:?}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Running application failed: {err:?}");
        process::exit(1);
    };
}
