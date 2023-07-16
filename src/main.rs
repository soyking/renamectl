#![feature(unix_chown)]

mod error;
mod file;
mod key;
mod run;

use crate::run::run;
use std::process::exit;

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => info!("done"),
        Err(e) => {
            error!("Failed: {:?}", e);
            exit(1);
        }
    };
}
