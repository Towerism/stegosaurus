# Stegosaurus

> A steganographic binary generation tool

The goal of stegosaurus is to be able to hide data in various binary forms:

- images,
- sound files,
- etc.

NOTE: tested and working with with .png and .bmp files. Tested and does not
work with JPEG files.

## Roadmap

- Retrieving data from images
- Encrypting data with passphrase
- Support for sound files (.wav files only)
- Add more steganographic methods (e.g. customizing how many LSBs).
- Probably more to come

## Development

Build using `cargo build`, and run tests with `cargo test`

## License

Copyright (C) 2018 Martin Fracker, Jr.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
