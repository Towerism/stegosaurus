#[macro_use]
extern crate clap;
extern crate image;

pub mod config;
pub mod img;
mod chunker;
mod lsb;

pub trait Embed {
    fn embed_data(&self, data: Vec<u8>) -> Box<dyn Save>;
}

pub trait Save {
    fn save(&self, path: &str) -> std::io::Result<()>;
}
