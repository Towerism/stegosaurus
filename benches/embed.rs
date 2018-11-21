#![feature(test)]

extern crate stegosaurus;
extern crate test as testing;

use testing::Bencher;
use stegosaurus::test;

#[bench]
fn embed_small_data_in_small_image(b: &mut Bencher) {
    b.iter(|| {
        test::embed(
            "small.bmp",
            "test_input.txt",
            "output/small_data_small_image.bmp",
            "test_passphrase"
        );
    })
}

#[bench]
fn embed_small_data_in_large_image(b: &mut Bencher) {
    b.iter(|| {
        test::embed(
            "large.png",
            "test_input.txt",
            "output/small_data_large_image.bmp",
            "test_passphrase"
        );
    })
}

#[bench]
fn embed_large_data_in_large_image(b: &mut Bencher) {
    b.iter(|| {
        test::embed(
            "large.png",
            "test_large_input.txt",
            "output/large_data_large_image.bmp",
            "test_passphrase"
        );
    })
}
