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
    let Config { cover, output, input } = config;

    let mut payload = Vec::new();
    let mut reader: Box<dyn Read> = match input {
        Some(i) => Box::new(fs::File::open(i)?),
        None => Box::new(io::stdin())
    };
    reader.read_to_end(&mut payload)?;
    let (payload, iv) = encryption::encrypt_payload(&payload)?;
    let payload = Payload::new(payload, iv)?;
    let payload = payload.bytes();

    let img = ImageCover::new(&cover)?;

    let final_img = img.embed_data(payload)?;
    final_img.save(&output)?;
    Ok(())
}

pub fn extract(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { cover, output, .. } = config;

    let img = ImageCover::new(&cover)?;
    let bytes = img.extract_data();
    let payload = Payload::from_bytes(bytes);
    let (payload, iv) = payload.data()?;
    let payload = encryption::decrypt_payload(payload, &iv)?;

    let mut buffer = fs::File::create(&output)?;
    buffer.write(&payload)?;

    Ok(())
}
