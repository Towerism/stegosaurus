use std::error::Error;
use std::fmt;

pub enum Operation {
    Embed(Config),
    Extract(Config)
}

pub struct Config {
    pub cover: String,
    pub output: String,
}

impl Config {
    fn new(matches: &clap::ArgMatches) -> Config {
        let cover = matches.value_of("cover").unwrap().to_string();
        let output = matches.value_of("output").unwrap().to_string();
        Config {
            cover,
            output,
        }
    }
}

impl Operation {
    pub fn new() -> Result<Operation, Box<dyn Error>> {
        let args = vec![
            clap::Arg::from_usage("-c --cover=<COVER> 'Sets the file to use as the steganographic cover'"),
            clap::Arg::from_usage("-o --output=<OUTPUT> 'Sets the path to use as the output'")
        ];

        let matches = clap::App::new("stegosaurus")
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .subcommand(clap::SubCommand::with_name("embed")
                .about("Embed data into a steganographic binary")
                .args(&args))
            .subcommand(clap::SubCommand::with_name("extract")
                .about("Extract data from a steganographic binary")
                .args(&args))
            .get_matches();

        match matches.subcommand() {
            ("embed", matches) => Ok(Operation::Embed(Config::new(&matches.unwrap()))),
            ("extract", matches) => Ok(Operation::Extract(Config::new(&matches.unwrap()))),
            _ => return Err(Box::new(ConfigError::new("error parsing subcommand")))
        }
    }
}

#[derive(Debug)]
struct ConfigError {
    message: String
}
impl ConfigError {
    fn new(message: &str) -> ConfigError {
        ConfigError {
            message: String::from(message)
        }
    }
}
impl Error for ConfigError {}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}
