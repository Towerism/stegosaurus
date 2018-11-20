# Stegosaurus

> A steganographic binary generation tool

The goal of stegosaurus is to be able to hide data in various binary forms:

- images,
- sound files,
- etc.

Should work with all image types, though only tested with bmp/jpg/png.

NOTE: When embedding with a non-bitmap cover, you should specify the output
with a bitmap-type extension (i.e. png or bmp). Due to lossy compression with
jpg files, your data will be unrecoverable since this tool only does LSB at the
moment.

## Requirements

Besides the rust toolchain, which is only necessary for building, you will need
the following installed on your system:

- openssl

## Example usage

Here is an example of embedding plain text in a png file. Note that the message will
be encrypted before being embedded.

```
$ echo "super secret message" | ./stegosaurus embed --cover ~/png-files-download-6.png --output innocuous.png
passphrase: 
confirm:

$ ./stegosaurus extract --cover innocuous.png --output message.txt

$ cat message.txt
super secret message
```

## Roadmap

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
