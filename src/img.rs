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
        let mut buffer = self.image.to_rgba().into_raw();
        for pixel in buffer.iter_mut() {
            *pixel = match data_encoder.encode_next(*pixel) {
                lsb::EncodeResult::Encoded(encoded) => encoded,
                lsb::EncodeResult::NotEncoded(_) => break
            };
        }
        Box::new(ImageFinal {
            buffer: ImageBuffer::from_raw(width, height, buffer).unwrap()
        })
    }
}

impl Extract for ImageBase {
    fn extract_data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let data_decoder = lsb::Decoder::new();
        let bytes = self.image.to_rgba().into_raw();
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
