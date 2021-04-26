extern crate nalgebra as na;
extern crate parry3d as pa;
mod camera;
mod film;
mod material;
mod math;
mod renderer;
mod scene;
mod shape;
mod sphere;
use std::sync::Arc;
use std::path::Path;

fn main() {
    let objects = vec![
        Arc::new(sphere::Sphere::new(
            1.,
            na::Isometry3::identity(),
            Arc::new(material::Lambert {
                color: na::Vector3::new(1., 0., 0.),
            }),
        )) as Arc<dyn shape::Shape>,
        Arc::new(sphere::Sphere::new(
            2.,
            na::Isometry3::translation(-2., -1.5, -5.),
            Arc::new(material::Metal {
                color: na::Vector3::new(0.95, 0.95, 0.95),
            }),
        )) as Arc<dyn shape::Shape>,
        Arc::new(sphere::Sphere::new(
            200.,
            na::Isometry3::translation(1., 200.5, -10.),
            Arc::new(material::Lambert {
                color: na::Vector3::new(0., 1., 0.),
            }),
        )) as Arc<dyn shape::Shape>,
    ];

    let scene = Arc::new(scene::Scene { shapes: objects });
    let width: u32 = 1024;
    let height: u32 = 768;
    let samples = 1000;
    let film = film::Film::new(width, height, samples, Path::new("image.png").to_path_buf());
    let eye = na::Point3::new(0f32, 0f32, 5f32);
    let target = na::Point3::new(0f32, 0f32, 0f32);
    let view = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());
    let camera = Arc::new(camera::Camera::new(view.to_matrix(), 3.14 / 2.0, &film));
    let mut renderer = renderer::Renderer{film: film, camera: camera, scene: scene};
    renderer.render();

}
