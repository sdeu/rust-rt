use super::math::Vec3;
use image::{Rgb, RgbImage};
use std::f64;
use std::path::PathBuf;

pub struct Film {
    filename: PathBuf,
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    lambda: f64,
    image: RgbImage,
}

impl Film {
    pub fn new(width: u32, height: u32, samples: u32, filename: PathBuf) -> Film {
        Film {
            filename: filename,
            width: width,
            height: height,
            samples: samples,
            lambda: 2.2,
            image: RgbImage::new(width, height),
        }
    }

    pub fn set_pixel(&mut self, u: u32, v: u32, color: Vec3) {
        let c = Rgb([
            (color.x.powf(self.lambda) * 255.) as u8,
            (color.y.powf(self.lambda) * 255.) as u8,
            (color.z.powf(self.lambda) * 255.) as u8,
        ]);
        self.image.put_pixel(u, v, c);
    }

    pub fn save(&self) {
        self.image.save(self.filename.as_path()).unwrap();
    }
}
