use std::fs;
use std::io;
use std::io::{Read, Write};
use std::error::Error;

use super::payload::Payload;
use super::config::Config;
use super::encryption;
use ::img::ImageCover;
use ::core::{Embed, Extract};

pub fn embed(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)?;
    let (payload, iv) = encryption::encrypt_payload(&payload)?;
    let payload = Payload::new(payload, iv)?;
    let payload = payload.bytes();

    let img = ImageCover::new(&filename)?;

    let final_img = img.embed_data(payload)?;
    final_img.save(&output)?;
    Ok(())
}

pub fn extract(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    let img = ImageCover::new(&filename)?;
    let bytes = img.extract_data();
    let payload = Payload::from_bytes(bytes);
    let (payload, iv) = payload.data()?;
    let payload = encryption::decrypt_payload(payload, &iv)?;

    let mut buffer = fs::File::create(&output)?;
    buffer.write(&payload)?;

    Ok(())
}
