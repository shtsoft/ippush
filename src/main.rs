use ippush::Config;

use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Setting up application failed: {err:?}");
        process::exit(1);
    });

    if let Err(err) = ippush::run(config) {
        eprintln!("Running application failed: {err:?}");
        process::exit(1);
    };
}
