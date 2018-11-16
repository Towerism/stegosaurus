use std::io;
use std::io::Read;
use std::process;
use std::error::Error;
use std::env;

pub struct Config {
    pub filename: String,
    pub payload: Vec<u8>
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let matches = clap::App::new("stegosaurus")
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .arg(clap::Arg::from_usage("-f --file=<FILE> 'Sets the file to use as the base of the steganographic binary'"))
            .get_matches_safe()?;

        let filename = matches.value_of("file").unwrap().to_string();

        let mut payload = Vec::new();
        io::stdin().read_to_end(&mut payload)?;

        Ok(Config {
            filename,
            payload
        })
    }
}
