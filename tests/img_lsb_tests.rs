extern crate stegosaurus;

use std::fs;
use std::io::Read;
use stegosaurus::test;

#[test]
fn img_and_lsb_preserve_message() {
    test::embed(
        "small.bmp",
        "test_input.txt",
        "output/test_stega.bmp",
        "test_passphrase",
    );
    test::extract(
        "output/test_stega.bmp",
        "test_input_extracted.txt",
        "test_passphrase",
    );

    let mut input_file = fs::File::open(test::resource("test_input.txt")).unwrap();
    let mut extracted_input_file =
        fs::File::open(test::resource("test_input_extracted.txt")).unwrap();
    let mut input = Vec::new();
    let mut extracted_input = Vec::new();
    input_file.read_to_end(&mut input).unwrap();
    extracted_input_file
        .read_to_end(&mut extracted_input)
        .unwrap();
    assert_eq!(input, extracted_input);
}
