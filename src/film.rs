use image::{GenericImage, RgbImage};
use std::path::PathBuf;

pub struct Film {
    filename: PathBuf,
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    image: RgbImage,
}

impl Film {
    pub fn new(width: u32, height: u32, samples: u32, filename: PathBuf) -> Film {
        Film {
            filename: filename,
            width: width,
            height: height,
            samples: samples,
            image: RgbImage::new(width, height),
        }
    }

    pub fn set_line(&mut self, line: &RgbImage, y: u32) {
        self.image.copy_from(line, 0, y).unwrap();
    }

    pub fn save(&self) {
        self.image.save(self.filename.as_path()).unwrap();
    }
}
