extern crate stegosaurus;
extern crate image;

use std::process;
use image::GenericImageView;

use stegosaurus::config::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });
    let Config { filename, payload } = config;

    let payload = String::from_utf8(payload).unwrap();

    let img = image::open(filename).unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });

    println!("Image dimensions: {:?}", img.dimensions());

    println!("Using payload: {}", payload);
}
