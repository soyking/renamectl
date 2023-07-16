#![feature(unix_chown)]

mod error;
mod file;
mod key;
mod run;

use clap::Parser;
use std::process::exit;

fn main() {
    let c = run::Config::parse();
    // debug
    println!("get config: {:?}", c);

    match run::run(&c) {
        Ok(_) => println!("done"),
        Err(e) => {
            println!("catch some error: {:?}", e);
            exit(1);
        }
    };
}
