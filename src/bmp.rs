use super::{Embed, Save};
use std::error::Error;
use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    RgbaImage
};

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
    fn embed_data(&self, data: &[u8]) -> Box<dyn Save> {
        let (width, height) = self.image.dimensions();
        let mut buffer = ImageBuffer::from_fn(width, height, |x, y| {
            self.image.get_pixel(x, y)
        });
        for pixel in buffer.pixels_mut() {
            for i in 0..4 {
                pixel[i] &= 0b0010_0000;
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
