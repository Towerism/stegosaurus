extern crate stegosaurus;
extern crate image;

use std::error::Error;
use std::process;
use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    RgbaImage
};

use stegosaurus::config::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });
    let Config { filename, payload } = config;


    let img = BmpBase::new(&filename).unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });

    let final_img = img.embed_data(&payload);
    final_img.save("result.bmp").unwrap_or_else(|err| {
        eprintln!("stegosaurus: {}", err);
        process::exit(1);
    });

    let payload = String::from_utf8(payload).unwrap();
    println!("Used payload: {}", payload);


}

trait Embed {
    fn embed_data(&self, data: &[u8]) -> Box<dyn Save>;
}

trait Save {
    fn save(&self, path: &str) -> std::io::Result<()>;
}

struct BmpBase {
    image: DynamicImage
}

impl BmpBase {
    fn new(path: &str) -> Result<BmpBase, Box<dyn Error>> {
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
