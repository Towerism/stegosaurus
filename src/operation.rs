use std::io;
use std::fs;
use std::io::{Read, Write};
use std::error::Error;

use super::payload::Payload;
use super::config::Config;
use super::img::ImageBase;
use super::Embed;
use super::Extract;

pub fn embed(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)?;
    let payload = Payload::new(payload)?;
    let payload = payload.bytes();

    let img = ImageBase::new(&filename)?;

    let final_img = img.embed_data(payload);
    final_img.save(&output)?;

    Ok(())
}

pub fn extract(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    let img = ImageBase::new(&filename)?;
    let bytes = img.extract_data();
    let payload = Payload::from_bytes(bytes);
    let data = payload.data()?;

    let mut buffer = fs::File::create(&output)?;
    buffer.write(&data)?;

    Ok(())
}
