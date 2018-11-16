#[macro_use]
extern crate clap;

extern crate stegosaurus;


use std::process;

use stegosaurus::config::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });
    let Config { filename, payload } = config;

    let payload = String::from_utf8(payload).unwrap();

    println!("Using file: {}", filename);
    println!("Using payload: {}", payload);
}
