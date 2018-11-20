mod config;
mod crypto;
mod operation;
mod payload;

use std::process;

use self::config::Operation;

pub fn run(argv: Vec<String>) {
    let op = Operation::new(argv).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    match op {
        Operation::Embed(config) => {
            operation::embed(&config).unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });
        }

        Operation::Extract(config) => {
            operation::extract(&config).unwrap_or_else(|err| {
                eprintln!("error while extracting: {}", err);
                process::exit(1);
            });
        }
    }
}

pub trait Embed {
    fn embed_data(&self, data: Vec<u8>) -> Result<Box<dyn Save>, EmbedError>;
}

pub trait Save {
    fn save(&self, path: &str) -> std::io::Result<()>;
}

pub trait Extract {
    fn extract_data(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct EmbedError {
    message: String,
}

impl EmbedError {
    pub fn new(message: &str) -> EmbedError {
        EmbedError {
            message: message.to_string(),
        }
    }
}

impl std::error::Error for EmbedError {}

impl std::fmt::Display for EmbedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Embedding error ({})", self.message)
    }
}
