extern crate nalgebra as na;
mod camera;
mod film;
mod intersection;
mod material;
mod math;
mod ray;
mod renderer;
mod scene;
mod shape;
mod sphere;
use std::rc::Rc;
use std::path::Path;

fn main() {
    let objects = vec![
        Box::new(sphere::Sphere::new(
            1.,
            na::Matrix4::new_translation(&na::Vector3::new(1., -0.5, -5.)),
            Rc::new(material::Lambert {
                color: na::Vector3::new(1., 0., 0.),
            }),
        )) as Box<dyn shape::Shape>,
        Box::new(sphere::Sphere::new(
            2.,
            na::Matrix4::new_translation(&na::Vector3::new(-2., -1.5, -5.)),
            Rc::new(material::Lambert {
                color: na::Vector3::new(0.9, 0.9, 0.9),
            }),
        )) as Box<dyn shape::Shape>,
        Box::new(sphere::Sphere::new(
            200.,
            na::Matrix4::new_translation(&na::Vector3::new(1., 200.5, -10.)),
            Rc::new(material::Lambert {
                color: na::Vector3::new(0., 1., 0.),
            }),
        )) as Box<dyn shape::Shape>,
    ];

    let scene = scene::Scene { shapes: objects };
    let film = film::Film::new(640, 480, 10, Path::new("/tmp/image.png").to_path_buf());
    let camera = camera::Camera::new(640, 480);
    let mut renderer = renderer::Renderer{film: film, camera: camera, scene: scene};
    renderer.render();

}
