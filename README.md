# Stegosaurus

> A steganographic binary generation tool

The goal of stegosaurus is to be able to hide data in various binary forms:

- images,
- sound files,
- etc.

## Roadmap

- Basic functionality (least-significant-bit, text-only), hiding data in images (.bmp files only), retrieving data from images
- Encrypting data with passphrase
- Support for sound files (.wav files only)
- Add more steganographic methods (e.g. customizing how many LSBs).
- Probably more to come

## Development

Build using `cargo build`, and run tests with `cargo test`
