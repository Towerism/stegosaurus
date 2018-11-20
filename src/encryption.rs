use std::io;
use std::fmt;
use std::io::Read;
use std::error::Error;
use openssl::symm;

use super::InitializationVector;

pub fn get_payload_and_encrypt() -> Result<(Vec<u8>, InitializationVector), Box<dyn Error>> {
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

pub fn decrypt_payload(payload: &[u8], iv: &InitializationVector) -> Result<Vec<u8>, Box<dyn Error>> {
    let pass = read_passphrase_from_tty("passphrase: ")?;
    let pass = pass.as_bytes();
    let mut key = [0; 16];
    openssl::pkcs5::pbkdf2_hmac(&pass, iv, 3, openssl::hash::MessageDigest::md5(), &mut key)?;
    let cipher = symm::Cipher::aes_128_cbc();
    let decrypted = symm::decrypt(cipher, &key, Some(iv), &payload)?;
    Ok(decrypted)
}

#[derive(Debug)]
pub struct PassphraseError {
    message: &'static str
}

impl Error for PassphraseError {}

impl fmt::Display for PassphraseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "passphrase error ({})", self.message)
    } 
}
