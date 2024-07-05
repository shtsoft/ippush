//! ippush is a remote procedure returning the callers IP address and a timestamp.

use std::env::Args;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use chrono::prelude::Utc;

use serde::Serialize;

/// Abstracts the configuration of the application.
pub struct Config {
    /// socket address to which the application binds
    addr: SocketAddr,
}

impl Config {
    /// Creates a new configuration of the application.
    /// - `args` are the commandline parameters made up by the IP and port to which the application should bind.
    ///
    ///  # Errors
    ///
    ///  An error is returned if the number of arguments is wrong or if parsing the arguments fails.
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

/// Abstracts the return value of the remote procedure.
#[derive(Serialize)]
struct ReturnValue {
    ip: IpAddr,
    timestamp: String,
}

/// Runs the application.
/// - `config` is the configuration the application is run in.
///
///  # Errors
///
///  An error is returned if writing or reading the socket fails.
#[allow(clippy::needless_pass_by_value)]
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(config.addr)?;

    let mut buf = [0; 0];

    loop {
        let (_, addr) = socket.recv_from(&mut buf)?;

        let return_value = serde_json::to_string(&ReturnValue {
            ip: addr.ip(),
            timestamp: Utc::now().to_string(),
        })?;

        socket.send_to(return_value.as_bytes(), addr)?;
    }
}
