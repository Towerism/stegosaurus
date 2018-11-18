#[macro_use]
extern crate clap;
extern crate image;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

pub mod config;
pub mod img;
mod chunker;
mod lsb;
mod payload;

pub trait Embed {
    fn embed_data(&self, data: Vec<u8>) -> Box<dyn Save>;
}

pub trait Save {
    fn save(&self, path: &str) -> std::io::Result<()>;
}
