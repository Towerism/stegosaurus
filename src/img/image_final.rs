use core::Save;

pub struct ImageFinal {
    buffer: image::RgbaImage,
}

impl ImageFinal {
    pub fn new(buffer: image::RgbaImage) -> ImageFinal {
        ImageFinal { buffer }
    }
}

impl Save for ImageFinal {
    fn save(&self, path: &str) -> std::io::Result<()> {
        self.buffer.save(path)
    }
}
