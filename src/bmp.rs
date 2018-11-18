use std::error::Error;
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView,
    ImageBuffer,
    RgbaImage
};

use super::lsb;
use super::{Embed, Save};

pub struct BmpBase {
    image: DynamicImage
}

impl BmpBase {
    pub fn new(path: &str) -> Result<BmpBase, Box<dyn Error>> {
        let image = image::open(path)?;

        Ok(BmpBase {
            image
        })
    }
}

struct BmpFinal {
    buffer: RgbaImage
}

impl Embed for BmpBase {
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
        Box::new(BmpFinal {
            buffer
        })
    }
}

impl Save for BmpFinal {
    fn save(&self, path: &str) -> std::io::Result<()> {
        self.buffer.save(path)
    }
}
