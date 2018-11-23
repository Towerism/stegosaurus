use openssl::symm;
use openssl::hash::MessageDigest;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

pub type InitializationVector = [u8; 16];
type EncryptionKey = [u8; 32];

pub struct Crypter {
    passphrase_reader: Option<io::BufReader<fs::File>>,
    require_confirm: bool,
}

impl Crypter {
    pub fn new(passphrase_filename: Option<String>) -> Result<Crypter, Box<dyn Error>> {
        Ok(Crypter {
            passphrase_reader: match passphrase_filename {
                Some(filename) => Some(io::BufReader::new(fs::File::open(&filename).map_err(
                    |err| {
                        Box::new(EncryptionError::new(&format!(
                            "couldn't open passfile: {}",
                            err
                        )))
                    },
                )?)),
                None => None,
            },
            require_confirm: false,
        })
    }

    pub fn require_passphrase_confirm(mut self) -> Self {
        self.require_confirm = true;
        self
    }

    pub fn encrypt_payload(
        self,
        payload: &[u8],
    ) -> Result<(Vec<u8>, InitializationVector), Box<dyn Error>> {
        let mut iv = [0; 16];
        openssl::rand::rand_bytes(&mut iv)?;
        let passphrase = self.read_passphrase_from_reader_or_tty()?;
        let key = Crypter::derive_key(&passphrase, &iv)?;
        let payload = symm::encrypt(Crypter::cipher(), &key, Some(&iv), &payload)?;
        Ok((payload, iv))
    }

    fn read_passphrase_from_reader_or_tty(self) -> Result<String, Box<dyn Error>> {
        let passphrase = match self.passphrase_reader {
            None => {
                let passphrase = rpassword::read_password_from_tty(Some("passphrase: "))?;
                let confirm = if self.require_confirm {
                    Some(rpassword::read_password_from_tty(Some("confirm: "))?)
                } else {
                    None
                };
                if self.require_confirm && confirm.unwrap() != passphrase {
                    return Err(Box::new(EncryptionError::new("passphrases did not match")));
                }
                passphrase
            }
            reader => rpassword::read_password_with_reader(reader).map_err(|err| {
                Box::new(EncryptionError::new(&format!(
                    "couldn't read passfile: {}",
                    err
                )))
            })?,
        };
        Ok(passphrase)
    }

    fn derive_key(passphrase: &str, iv: &[u8]) -> Result<EncryptionKey, Box<dyn Error>> {
        let mut key = [0; 32];
        let pass = passphrase.as_bytes();
        openssl::pkcs5::pbkdf2_hmac(&pass, &iv, 3, MessageDigest::md5(), &mut key)
            .map_err(|_| Box::new(EncryptionError::new("failed to generate AES key")))?;
        Ok(key)
    }

    fn cipher() -> symm::Cipher {
        symm::Cipher::aes_256_cbc()
    }

    pub fn decrypt_payload(
        self,
        payload: &[u8],
        iv: &InitializationVector,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let passphrase = self.read_passphrase_from_reader_or_tty()?;
        let key = Crypter::derive_key(&passphrase, iv)?;
        let decrypted = symm::decrypt(Crypter::cipher(), &key, Some(iv), &payload)
            .map_err(|_| Box::new(EncryptionError::new("failed to decrypt payload")))?;
        Ok(decrypted)
    }
}

#[derive(Debug)]
pub struct EncryptionError {
    message: String,
}

impl EncryptionError {
    pub fn new(message: &str) -> EncryptionError {
        EncryptionError {
            message: message.to_string(),
        }
    }
}

impl Error for EncryptionError {}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "encryption/decryption error ({})", self.message)
    }
}
