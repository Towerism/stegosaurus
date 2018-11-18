use std::io;
use std::io::Read;
use std::error::Error;

use super::payload::Payload;

pub struct Config {
    pub filename: String,
    pub output: String,
    pub payload: Vec<u8>
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let matches = clap::App::new("stegosaurus")
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .arg(clap::Arg::from_usage("-f --file=<FILE> 'Sets the file to use as the base of the steganographic binary'"))
            .arg(clap::Arg::from_usage("-o --output=<OUTPUT> 'Sets the path to use as the final steganographic binary'"))
            .get_matches_safe()?;

        let filename = matches.value_of("file").unwrap().to_string();
        let output = matches.value_of("output").unwrap().to_string();

        let mut payload = Vec::new();
        io::stdin().read_to_end(&mut payload)?;
        let payload = Payload::new(payload)?;
        let payload = payload.bytes();

        Ok(Config {
            filename,
            output,
            payload
        })
    }
}
