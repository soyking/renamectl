#![feature(unix_chown)]

mod error;
mod file;
mod key;
mod run;

use clap::Parser;
use std::process::exit;

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    let c = run::Config::parse();
    debug!("get config: {:?}", c);

    match run::run(&c) {
        Ok(_) => info!("done"),
        Err(e) => {
            error!("Failed: {:?}", e);
            exit(1);
        }
    };
}
