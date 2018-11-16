#[macro_use]
extern crate clap;
use std::io;
use std::io::Read;
use std::process;

fn main() {
    let matches = clap::App::new("stegosaurus")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(clap::Arg::from_usage("-f --file=<FILE> 'Sets the file to use as the base of the steganographic binary'"))
        .get_matches();

    if let Some(file) = matches.value_of("file") {
        println!("Using file: {}", file);
    }

    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)
        .expect("Failed to read payload");

    let payload = match String::from_utf8(payload) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid UTF-8 sequence: {}", e);
            process::exit(1);
        }
    };

    println!("Using payload: {}", payload);
}
