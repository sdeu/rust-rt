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
        match self.image.copy_from(line, 0, y) {
            Ok(_) => (),
            Err(_) => println!("Error storing scanline")
        }
    }

    pub fn save(&self) {
        match self.image.save(self.filename.as_path()) {
            Ok(_) => println!("Image save successfuly"),
            Err(_) => println!("Error saving the image")
        }
    }
}
