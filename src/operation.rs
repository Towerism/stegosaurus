use std::io;
use std::fs;
use std::fmt;
use std::io::{Read, Write, Seek, SeekFrom};
use std::error::Error;

use super::payload::Payload;
use super::config::Config;
use super::img::ImageBase;
use super::Embed;
use super::Extract;

pub fn embed(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    let payload = get_payload_and_encrypt()?;
    let payload = Payload::new(payload)?;
    let payload = payload.bytes();

    let img = ImageBase::new(&filename)?;

    let final_img = img.embed_data(payload);
    final_img.save(&output)?;

    Ok(())
}

fn get_payload_and_encrypt() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)?;
    let passphrase = get_passphrase()?;
    Ok(payload)
}

fn get_passphrase() -> Result<String, Box<dyn Error>> {
    let passphrase = read_passphrase_from_tty("passphrase: ")?;
    let confirm = read_passphrase_from_tty("confirm: ")?;
    if passphrase != confirm {
        return Err(Box::new(PassphraseError { message: "Passphrases did not match" }));
    }
    Ok(passphrase)
}

fn read_passphrase_from_tty(prompt: &str) -> Result<String, Box<PassphraseError>> {
    if let Ok(p) = rpassword::read_password_from_tty(Some(prompt)) {
        Ok(p)
    } else {
        Err(Box::new(PassphraseError { message: "Could not read passphrase" }))
    }
}

pub fn extract(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    println!("Extracting from: {}", filename);
    let img = ImageBase::new(&filename)?;
    let bytes = img.extract_data();
    let payload = Payload::from_bytes(bytes);
    let data = payload.data()?;

    let mut buffer = fs::File::create(&output)?;
    buffer.write(&data)?;

    Ok(())
}

#[derive(Debug)]
struct PassphraseError {
    message: &'static str
}

impl Error for PassphraseError {}

impl fmt::Display for PassphraseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "passphrase error ({})", self.message)
    } 
}
