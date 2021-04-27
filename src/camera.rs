use super::film::Film;
use super::math::Matrix4;
use pa::query::Ray;
use std::f32;

pub struct Camera {
    pub camera_to_world: Matrix4,
    pub perspective: na::Perspective3<f32>,
    pub fov: f32,
    pub raster_to_screen: Matrix4,
    pub screen_to_raster: Matrix4,
}

impl Camera {
    pub fn new(world_to_camera: Matrix4, fov: f32, film: &Film) -> Camera {
        let camera_to_world = world_to_camera.try_inverse().unwrap();
        let aspect_ratio = film.width as f32 / film.height as f32;
        let perspective = na::Perspective3::new(aspect_ratio, fov, 1f32, 1000.0);

        let screen_to_raster = na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(
            film.width as f32,
            film.height as f32,
            1f32,
        )) * na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(
            0.5, 0.5, 1f32,
        )) * na::Matrix4::new_translation(&na::Vector3::new(1., 1., 0f32));

        let raster_to_screen = screen_to_raster.try_inverse().unwrap();

        Camera {
            camera_to_world: camera_to_world,
            perspective: perspective,
            fov: fov,
            raster_to_screen: raster_to_screen,
            screen_to_raster: screen_to_raster,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let far_ndc_point = self
            .raster_to_screen
            .transform_point(&na::Point3::new(u, v, 1f32));
        let far_view_point = self.perspective.unproject_point(&far_ndc_point);
        Ray {
            origin: self
                .camera_to_world
                .transform_point(&na::Point3::new(0f32, 0f32, 0f32)),
            dir: far_view_point.coords.normalize(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::film::Film;
    use super::Camera;
    use std::path::Path;

    #[test]
    fn test_midpoint_ray() {
        let film = Film::new(100, 100, 1, Path::new("test").to_path_buf());
        let eye = na::Point3::new(0f32, 0f32, 0f32);
        let target = na::Point3::new(0f32, 0f32, 1f32);
        let view = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());
        let camera = Camera::new(view.to_matrix(), std::f32::consts::FRAC_PI_2, &film);

        let ray = camera.ray(50., 50.);

        assert_eq! {ray.origin, na::Point3::new(0., 0., 0.)};
        assert_eq! {ray.dir, na::Vector3::new(0., 0., -1.)};
    }
}
