#[macro_use]
extern crate clap;
extern crate image;

pub mod config;
pub mod bmp;

pub trait Embed {
    fn embed_data(&self, data: &[u8]) -> Box<dyn Save>;
}

pub trait Save {
    fn save(&self, path: &str) -> std::io::Result<()>;
}
