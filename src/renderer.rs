use super::camera::Camera;
use super::film::Film;
use super::math::Vec3;
use super::shape::Shape;
use pa::query::Ray;
use pa::query::RayIntersection;
use super::scene::Scene;
use rand::Rng;
use std::f32;

pub struct Renderer {
    pub film: Film,
    pub camera: Camera,
    pub scene: Scene,
}

impl Renderer {
    pub fn render(&mut self) {
        let mut rng = rand::thread_rng();
        let mut j = 0;
        let mut total = 0.;
        let mut bar = progress::Bar::new();
        bar.set_job_title("Rendering...");
        while j < self.film.height {
            let mut i = 0;
            while i < self.film.width {
                let mut sample = 0;
                let mut color = na::Vector3::new(0., 0., 0.);
                while sample < self.film.samples {
                    let u: f32 = i as f32 + rng.gen::<f32>();
                    let v: f32 = j as f32 + rng.gen::<f32>();
                    let ray = self.camera.ray(u, v);
                    let c = self.color(&ray);
                    color += c;
                    sample += 1;
                }
                self.film.set_pixel(i, j, color / self.film.samples as f32);
                i += 1;
            }
            total += 100.0 / (self.film.height as f32);
            bar.reach_percent(total as i32);
            j += 1;
        }

        self.film.save();
    }

    fn color(&self, ray: &Ray) -> Vec3 {
        self.color_rec(ray, 50)
    }

    fn color_rec(&self, ray: &Ray, depth: i32) -> Vec3 {
        if depth < 0 {
            return na::Vector3::new(0., 0., 0.);
        }
        let mut min_t = f32::MAX;
        let mut min_hit: Option<RayIntersection> = None;
        let mut min_shape: Option<&Box<dyn Shape>> = None;
        for shape in &self.scene.shapes {
            let i = shape.hit(ray);
            if let Some(intersection) = i {
                if intersection.toi < min_t && intersection.toi > 0.{
                    min_t = intersection.toi;
                    min_hit = Some(intersection);
                    min_shape = Some(shape);
                }
            };
        }

        if let Some(h) = min_hit {
            if let Some(s) = min_shape {
                let mat = s.material_at(&h);
                if let Some(scattered_ray) = mat.scatter(ray, &h)
                {
                    return mat.color().component_mul(&self.color_rec(&scattered_ray, depth - 1));
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
}
