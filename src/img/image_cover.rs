use std::error::Error;

use core::{Embed, EmbedError, Extract, Save};
use lsb;

pub struct ImageCover {
    image: image::DynamicImage,
    data_encoder: Option<lsb::Encoder>
}

impl ImageCover {
    pub fn new(path: &str, data_encoder: Option<lsb::Encoder>) -> Result<ImageCover, Box<dyn Error>> {
        let image = image::open(path)?;

        Ok(ImageCover {
            image,
            data_encoder
        })
    }
}

impl Embed for ImageCover {
    fn embed_data(&self) -> Result<Box<dyn Save>, EmbedError> {
        let mut buffer = self.image.to_rgba();
        buffer.pixels_mut().enumerate().for_each(|(i, mut pixel)| {
            self.encode_data_in_pixel(i, &mut pixel);
        });
        Ok(Box::new(super::ImageFinal::new(buffer)))
    }
}

impl Extract for ImageCover {
    fn extract_data(&self) -> Vec<u8> {
        let bytes = self.image.to_rgb().into_raw();
        ImageCover::extract_data_from_buffer(&bytes)
    }
}

impl ImageCover {
    fn encode_data_in_pixel(&self, pixel_index: usize, pixel: &mut image::Rgba<u8>) -> bool {
        let data_encoder = self.data_encoder.as_ref().unwrap();
        for i in 0..3 {
            let subpixel = &mut pixel[i];
            let index = pixel_index * 3 + i;
            *subpixel = match data_encoder.encode_using_bit_at(*subpixel, index) {
                lsb::EncodeResult::Encoded(encoded) => encoded,
                lsb::EncodeResult::NotEncoded(_) => return false
            };
        }
        true
    }

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
