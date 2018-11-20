use std::io;
use std::fmt;
use std::io::Read;
use std::error::Error;
use openssl::symm;
use openssl::hash;

pub type EncryptionBlock = [u8; 16];

pub fn get_payload_and_encrypt() -> Result<(Vec<u8>, EncryptionBlock), Box<dyn Error>> {
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

fn encrypt_payload(payload: &[u8], passphrase: String) -> Result<(Vec<u8>, EncryptionBlock), Box<dyn Error>> {
    let mut iv = [0; 16];
    openssl::rand::rand_bytes(&mut iv)?;
    let key = derive_key(&passphrase, &iv)?;
    let payload = symm::encrypt(cipher(), &key, Some(&iv), &payload)?;
    Ok((payload, iv))
}

fn derive_key(passphrase: &str, iv: &[u8]) -> Result<EncryptionBlock, Box<dyn Error>> {
    let mut key = [0; 16];
    let pass = passphrase.as_bytes();
    openssl::pkcs5::pbkdf2_hmac(&pass, &iv, 3, digest(), &mut key)?;
    Ok(key)
}

fn cipher() -> symm::Cipher {
    symm::Cipher::aes_128_cbc()
}

fn digest() -> hash::MessageDigest {
    hash::MessageDigest::md5()
}

pub fn decrypt_payload(payload: &[u8], iv: &EncryptionBlock) -> Result<Vec<u8>, Box<dyn Error>> {
    let passphrase = read_passphrase_from_tty("passphrase: ")?;
    let key = derive_key(&passphrase, iv)?;
    let decrypted = symm::decrypt(cipher(), &key, Some(iv), &payload)?;
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
