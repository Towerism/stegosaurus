use std::error::Error;
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    ImageBuffer,
    RgbaImage
};

use super::lsb;
use super::{Embed, Save, Extract};

pub struct ImageBase {
    image: DynamicImage
}

impl ImageBase {
    pub fn new(path: &str) -> Result<ImageBase, Box<dyn Error>> {
        let image = image::open(path)?;

        Ok(ImageBase {
            image
        })
    }
}

struct ImageFinal {
    buffer: RgbaImage
}

impl Embed for ImageBase {
    fn embed_data(&self, data: Vec<u8>) -> Box<dyn Save> {
        let mut data_encoder = lsb::Encoder::new(data);
        let (width, height) = self.image.dimensions();
        let mut buffer = ImageBuffer::new(width, height);
        buffer.copy_from(&self.image, 0, 0);
        'outer: for pixel in buffer.pixels_mut() {
            for i in 0..4 {
                let subpixel = pixel[i];
                pixel[i] = match data_encoder.encode_next(subpixel) {
                    lsb::EncodeResult::Encoded(encoded) => encoded,
                    lsb::EncodeResult::NotEncoded(_) => break 'outer
                };
            }
        }
        Box::new(ImageFinal {
            buffer
        })
    }
}

impl Extract for ImageBase {
    fn extract_data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let data_decoder = lsb::Decoder::new();
        let bytes = self.image.raw_pixels();
        for chunk in bytes.chunks(8) {
            if chunk.len() < 8 {
                break;
            }
            let decoded = data_decoder.decode_next(chunk);
            result.push(decoded);
        }
        result
    }
}

impl Save for ImageFinal {
    fn save(&self, path: &str) -> std::io::Result<()> {
        self.buffer.save(path)
    }
}
