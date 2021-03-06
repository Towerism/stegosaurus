use std::error::Error;
use std::fmt;

pub enum Operation {
    Embed(Config),
    Extract(Config),
}

pub struct Config {
    pub cover: String,
    pub output: String,
    pub input: Option<String>,
    pub passfile: Option<String>,
}

impl Config {
    fn new(matches: &clap::ArgMatches) -> Config {
        let cover = matches.value_of("cover").unwrap().to_string();
        let output = matches.value_of("output").unwrap().to_string();
        let input = matches.value_of("input").map(|input| input.to_string());
        let passfile = matches
            .value_of("passfile")
            .map(|passfile| passfile.to_string());
        Config {
            cover,
            output,
            input,
            passfile,
        }
    }
}

impl Operation {
    pub fn new(argv: Vec<String>) -> Result<Operation, Box<dyn Error>> {
        let args = vec![
            clap::Arg::from_usage("-c --cover=<COVER> 'Sets the file to use as the steganographic cover'"),
            clap::Arg::from_usage("-o --output=<OUTPUT> 'Sets the path to use as the output'"),
            clap::Arg::from_usage("-p --passfile=[PASSFILE] 'Use the contents of PASSFILE instead of the tty for the passphrase'")
        ];

        let matches = clap::App::new("stegosaurus")
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .subcommand(clap::SubCommand::with_name("embed")
                .about("Embed INPUT (or stdin) into a steganographic cover")
                .arg(clap::Arg::from_usage("-i --input=[INPUT] 'Sets the file to be embeded in the steganographic cover'"))
                .args(&args))
            .subcommand(clap::SubCommand::with_name("extract")
                .about("Extract data from a steganographic cover")
                .args(&args))
            .get_matches_from(argv);

        match matches.subcommand() {
            ("embed", matches) => Ok(Operation::Embed(Config::new(&matches.unwrap()))),
            ("extract", matches) => Ok(Operation::Extract(Config::new(&matches.unwrap()))),
            _ => return Err(Box::new(ConfigError::new("error parsing subcommand"))),
        }
    }
}

#[derive(Debug)]
struct ConfigError {
    message: String,
}
impl ConfigError {
    fn new(message: &str) -> ConfigError {
        ConfigError {
            message: String::from(message),
        }
    }
}
impl Error for ConfigError {}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}
