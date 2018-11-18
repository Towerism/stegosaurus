extern crate stegosaurus;

use std::process;

use stegosaurus::config::{Operation};

use stegosaurus::operation;

fn main() {
    let op = Operation::new().unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    match op {
        Operation::Embed(config) => {
            operation::embed(&config).unwrap_or_else(|err| {
                eprintln!("error while embedding: {}", err);
            });
        },

        Operation::Extract(config) => {
            operation::extract(&config).unwrap_or_else(|err| {
                eprintln!("error while extracting: {}", err);
            });
        }
    }
}
