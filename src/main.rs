extern crate stegosaurus;

use std::process;

use stegosaurus::config::Config;
use stegosaurus::img::ImageBase;
use stegosaurus::Embed;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });
    let Config { filename, output, payload } = config;

    let img = ImageBase::new(&filename).unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });

    let final_img = img.embed_data(payload);
    final_img.save(&output).unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });


}
