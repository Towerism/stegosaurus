extern crate stegosaurus;

use std::iter::FromIterator;

fn main() {
    let argv = Vec::from_iter(std::env::args());
    stegosaurus::core::run(argv);
}
