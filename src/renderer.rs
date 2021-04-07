use super::camera::Camera;
use super::film::Film;
use super::math::Vec3;
use super::ray::Ray;
use super::scene::Scene;
use rand::Rng;

pub struct Renderer {
    pub film: Film,
    pub camera: Camera,
    pub scene: Scene,
}

impl Renderer {
    pub fn render(&mut self) {
        let mut rng = rand::thread_rng();
        let mut j = 0;
        let inc = 100.0 / (self.film.height * self.film.width * self.film.samples) as f64;
        let mut total = 0.;
        let mut bar = progress::Bar::new();
        bar.set_job_title("Rendering...");
        while j < self.film.height {
            let mut i = 0;
            while i < self.film.width {
                let mut sample = 0;
                let mut color = na::Vector3::new(0., 0., 0.);
                while sample < self.film.samples {
                    let u: f64 = (i as f64 + rng.gen::<f64>()) / (self.film.width as f64 - 1.);
                    let v: f64 = (j as f64 + rng.gen::<f64>()) / (self.film.height as f64 - 1.);
                    let ray = self.camera.ray(u, v);
                    let c = self.color(ray);
                    color += c;
                    sample += 1;
                    total += inc;
                    bar.reach_percent(total as i32)
                }
                self.film.set_pixel(i, j, color / self.film.samples as f64);
                i += 1;
            }

            j += 1;
        }

        self.film.save();
    }

    fn color(&self, ray: Ray) -> Vec3 {
        self.color_rec(ray, 5)
    }

    fn color_rec(&self, ray: Ray, depth: i32) -> Vec3 {
        if depth < 0 {
            return na::Vector3::new(0., 0., 0.);
        }
        na::Vector3::new(0., 0., 0.)
    }
}
