use std::error::Error;
use std::fs;
use std::io;
use std::io::{Read, Write};

use super::config::Config;
use super::crypto;
use super::payload::Payload;
use core::{Embed, Extract};
use img::ImageCover;

pub fn embed(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config {
        cover,
        output,
        input,
        passfile,
    } = config;

    let mut payload = Vec::new();
    let mut reader: Box<dyn Read> = match input {
        Some(i) => Box::new(fs::File::open(i)?),
        None => Box::new(io::stdin()),
    };
    reader.read_to_end(&mut payload)?;
    let crypter = crypto::Crypter::new(passfile.to_owned())?.require_passphrase_confirm();
    let (payload, iv) = crypter.encrypt_payload(&payload)?;
    let payload = Payload::new(payload, iv)?;
    let payload = payload.bytes();

    let img = ImageCover::new(&cover)?;

    let final_img = img.embed_data(payload)?;
    final_img.save(&output)?;
    Ok(())
}

pub fn extract(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config {
        cover,
        output,
        passfile,
        ..
    } = config;

    let img = ImageCover::new(&cover)?;
    let bytes = img.extract_data();
    let payload = Payload::from_bytes(bytes);
    let (payload, iv) = payload.data()?;
    let crypter = crypto::Crypter::new(passfile.to_owned())?;
    let payload = crypter.decrypt_payload(payload, &iv)?;

    let mut buffer = fs::File::create(&output)?;
    buffer.write(&payload)?;

    Ok(())
}
