use std::error::Error;
use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    RgbImage
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
    buffer: RgbImage
}

impl Embed for ImageBase {
    fn embed_data(&self, data: Vec<u8>) -> Box<dyn Save> {
        let mut data_encoder = lsb::Encoder::new(data);
        let (width, height) = self.image.dimensions();
        let mut buffer = self.image.to_rgb().into_raw();
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
        let bytes = self.image.to_rgb().into_raw();
        ImageBase::extract_data_from_buffer(&bytes)
    }
}

impl ImageBase {
    fn extract_data_from_buffer(buffer: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let data_decoder = lsb::Decoder::new();
        for chunk in buffer.chunks(8) {
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
