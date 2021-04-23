use super::camera::Camera;
use super::film::Film;
use super::math::Vec3;
use super::shape::Shape;
use pa::query::Ray;
use pa::query::RayIntersection;
use super::scene::Scene;
use rand::Rng;
use std::f32;
use image::{Rgb, RgbImage};

pub struct Renderer {
    pub film: Film,
    pub camera: Camera,
    pub scene: Scene,
}

impl Renderer {
    pub fn render(&mut self) {
        let mut total = 0.;
        let mut bar = progress::Bar::new();
        bar.set_job_title("Rendering...");
        for j in 0..self.film.height {
            let line = render_scanline(j, self.film.width, self.film.samples, 2.2, &self.scene, &self.camera);
            self.film.set_line(&line, j);
            total += 100.0 / (self.film.height as f32);
            bar.reach_percent(total as i32);
        }
        self.film.save();
    }
}

    fn color(ray: &Ray, scene: &Scene) -> Vec3 {
        color_rec(ray, scene, 50)
    }

    fn color_rec(ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
        if depth < 0 {
            return na::Vector3::new(0., 0., 0.);
        }
        let mut min_t = f32::MAX;
        let mut min_hit: Option<RayIntersection> = None;
        let mut min_shape: Option<&Box<dyn Shape>> = None;
        for shape in &scene.shapes {
            let i = shape.hit(ray);
            if let Some(intersection) = i {
                if intersection.toi < min_t && intersection.toi > 0.{
                    min_t = intersection.toi;
                    min_hit = Some(intersection);
                    min_shape = Some(&shape);
                }
            };
        }

        if let Some(h) = min_hit {
            if let Some(s) = min_shape {
                let mat = s.material_at(&h);
                if let Some(scattered_ray) = mat.scatter(ray, &h)
                {
                    return mat.color().component_mul(&color_rec(&scattered_ray, scene, depth - 1));
                }
            }
            return na::Vector3::zeros();
        }
        else {
            let d = ray.dir.normalize();
            let t = 0.5 * (d.y + 1.0);
            let sky = na::Vector3::new(0.5, 0.7, 1.0);
            return ((1.0 - t) * na::Vector3::new(1.0, 1.0, 1.0)) + (t * sky);       
        }
    }
    
    fn render_scanline(y_offset: u32, width: u32, samples: u32, lambda: f32, scene: &Scene, camera: &Camera) -> RgbImage {
        let mut rng = rand::thread_rng();
        let mut image = RgbImage::new(width, 1); 
        for i in 0..width {
            let mut col = na::Vector3::new(0., 0., 0.);
            for _ in 0..samples {
                let u: f32 = i as f32 + rng.gen::<f32>();
                let v: f32 = y_offset as f32 + rng.gen::<f32>();
                let ray = camera.ray(u, v);
                let c = color(&ray, scene);
                col += c;
            }

            col = col / samples as f32;
            image.put_pixel(i, 0, Rgb([
                (col.x.powf(lambda) * 255.) as u8,
                (col.y.powf(lambda) * 255.) as u8,
                (col.z.powf(lambda) * 255.) as u8,
            ]));
        }
        return image;
    }

