use std::io;
use std::fs;
use std::fmt;
use std::io::{Read, Write, Seek, SeekFrom};
use std::error::Error;
use openssl::symm;

use super::payload::Payload;
use super::config::Config;
use super::img::ImageBase;
use super::Embed;
use super::Extract;
use super::InitializationVector;

pub fn embed(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    let (payload, iv) = get_payload_and_encrypt()?;
    let payload = Payload::new(payload, iv)?;
    let payload = payload.bytes();

    let img = ImageBase::new(&filename)?;

    let final_img = img.embed_data(payload);
    final_img.save(&output)?;

    Ok(())
}

fn get_payload_and_encrypt() -> Result<(Vec<u8>, InitializationVector), Box<dyn Error>> {
    let mut payload = Vec::new();
    io::stdin().read_to_end(&mut payload)?;
    let passphrase = get_passphrase()?;
    let (payload, iv) = encrypt_payload(&payload, passphrase)?;
    Ok((payload, iv))
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

fn encrypt_payload(payload: &[u8], key: String) -> Result<(Vec<u8>, InitializationVector), Box<dyn Error>> {
    let cipher = symm::Cipher::aes_128_cbc();
    let pass = key.as_bytes();
    let mut iv = [0; 16];
    openssl::rand::rand_bytes(&mut iv)?;
    let mut key = [0; 16];
    openssl::pkcs5::pbkdf2_hmac(&pass, &iv, 3, openssl::hash::MessageDigest::md5(), &mut key)?;

    let payload = symm::encrypt(cipher, &key, Some(&iv), &payload)?;
    Ok((payload, iv))
}

pub fn extract(config: &Config) -> Result<(), Box<dyn Error>> {
    let Config { filename, output } = config;

    println!("Extracting from: {}", filename);
    let img = ImageBase::new(&filename)?;
    let bytes = img.extract_data();
    let payload = Payload::from_bytes(bytes);
    let (data, iv) = payload.data()?;
    let pass = read_passphrase_from_tty("passphrase: ")?;
    let pass = pass.as_bytes();
    let mut key = [0; 16];
    openssl::pkcs5::pbkdf2_hmac(&pass, &iv, 3, openssl::hash::MessageDigest::md5(), &mut key)?;
    let cipher = symm::Cipher::aes_128_cbc();
    let data = symm::decrypt(cipher, &key, Some(&iv), &data)?;

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
