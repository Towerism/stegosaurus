#[macro_use]
extern crate clap;
extern crate image;

extern crate openssl;
extern crate rpassword;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

pub mod core;
mod img;
mod lsb;
pub mod test;
