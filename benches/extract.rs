#![feature(test)]

extern crate stegosaurus;
extern crate test as testing;

use testing::Bencher;
use stegosaurus::test;

#[bench]
fn extract_small_data_in_small_image(b: &mut Bencher) {
    b.iter(|| {
        test::extract(
            "test_small_data_small_image.bmp",
            "output/extracted_test_input_from_small_image.txt",
            "test_passphrase"
        );
    })
}

#[bench]
fn extract_small_data_in_large_image(b: &mut Bencher) {
    b.iter(|| {
        test::extract(
            "test_small_data_large_image.bmp",
            "output/extracted_test_input_from_large_image.txt",
            "test_passphrase"
        );
    })
}

#[bench]
fn extract_large_data_in_large_image(b: &mut Bencher) {
    b.iter(|| {
        test::extract(
            "test_large_data_large_image.bmp",
            "output/extracted_test_large_input.txt",
            "test_passphrase"
        );
    })
}
