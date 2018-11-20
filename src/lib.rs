#[macro_use]
extern crate clap;
extern crate image;

extern crate openssl;
extern crate rpassword;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

pub mod config;
mod img;
mod payload;
pub mod operation;
mod chunker;
mod lsb;
mod encryption;

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
    message: String
}

impl EmbedError {
    pub fn new(message: &str) -> EmbedError {
        EmbedError {
            message: message.to_string()
        }
    }
}

impl std::error::Error for EmbedError {}

impl std::fmt::Display for EmbedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Embedding error ({})", self.message)
    }
}
