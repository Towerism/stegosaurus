use std::fmt;
use std::error::Error;
use openssl::symm;
use openssl::hash;

pub type InitializationVector = [u8; 16];
type EncryptionKey = [u8; 32];

pub fn encrypt_payload(payload: &[u8]) -> Result<(Vec<u8>, InitializationVector), Box<dyn Error>> {
    let mut iv = [0; 16];
    openssl::rand::rand_bytes(&mut iv)?;
    let passphrase = get_passphrase()?;
    let key = derive_key(&passphrase, &iv)?;
    let payload = symm::encrypt(cipher(), &key, Some(&iv), &payload)?;
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

fn derive_key(passphrase: &str, iv: &[u8]) -> Result<EncryptionKey, Box<dyn Error>> {
    let mut key = [0; 32];
    let pass = passphrase.as_bytes();
    openssl::pkcs5::scrypt(&pass, &iv, 0x4000, 8, 1, 0x20000000, &mut key)?;
    Ok(key)
}

fn cipher() -> symm::Cipher {
    symm::Cipher::aes_256_cbc()
}

pub fn decrypt_payload(payload: &[u8], iv: &InitializationVector) -> Result<Vec<u8>, Box<dyn Error>> {
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
