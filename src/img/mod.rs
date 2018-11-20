use std::error::Error;

use super::lsb;
use ::core::{Embed, Save, Extract, EmbedError};

pub struct ImageCover {
    image: image::DynamicImage
}

impl ImageCover {
    pub fn new(path: &str) -> Result<ImageCover, Box<dyn Error>> {
        let image = image::open(path)?;

        Ok(ImageCover {
            image
        })
    }
}

struct ImageFinal {
    buffer: image::RgbaImage
}

impl Embed for ImageCover {
    fn embed_data(&self, data: Vec<u8>) -> Result<Box<dyn Save>, EmbedError> {
        let mut buffer = self.image.to_rgba();
        if buffer.len() / 4 * 3 < data.len() {
            return Err(EmbedError::new("cover image is too small"));
        }
        let mut data_encoder = lsb::Encoder::new(data);
        'outer: for pixel in buffer.pixels_mut() {
            for i in 0..3 {
                let subpixel = &mut pixel[i];
                *subpixel = match data_encoder.encode_next(*subpixel) {
                    lsb::EncodeResult::Encoded(encoded) => encoded,
                    lsb::EncodeResult::NotEncoded(_) => break 'outer
                };
            }
        }
        Ok(Box::new(ImageFinal {
            buffer
        }))
    }
}

impl Extract for ImageCover {
    fn extract_data(&self) -> Vec<u8> {
        let bytes = self.image.to_rgb().into_raw();
        ImageCover::extract_data_from_buffer(&bytes)
    }
}

impl ImageCover {
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
