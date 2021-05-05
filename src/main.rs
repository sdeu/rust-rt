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
pub mod file;
use std::path::Path;
use std::sync::Arc;
use rand::prelude::*;

fn main() {

    let mut rng = thread_rng();
    let mut positions = vec![Arc::new(na::Vector3::new(0., 0., 0.))];
    while positions.len() < 70 {
        let candidate = Arc::new(na::Vector3::new(
            rng.gen_range(-20.0..20.0), 
            rng.gen_range(0.3..2.0), 
            rng.gen_range(-10.0..10.0)));
        if !positions.iter().any(|v| {
            candidate.metric_distance(v) <= 2.0
        })
        {
            positions.push(candidate);
        }
    }

    let mut objects = Vec::new();
    for pos in positions{
        if rng.gen() {
            let material = Arc::new(material::Metal {
                color: na::Vector3::new(1., 1., 1.)
            });
            let sphere = Arc::new(sphere::Sphere::new(
                1.,
                na::Isometry3::translation(pos.x, pos.y, pos.z),
                material
            ));
            objects.push(sphere as Arc<dyn shape::Shape>);
        }
        else {
            let material = Arc::new(material::Lambert {
                color: na::Vector3::new(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0)
                )
            });
            let sphere = Arc::new(sphere::Sphere::new(
                1.,
                na::Isometry3::translation(pos.x, pos.y, pos.z),
                material
            ));
            objects.push(sphere as Arc<dyn shape::Shape>);

        }
    };
    objects.push(
        Arc::new(sphere::Sphere::new(
            300.,
            na::Isometry3::translation(1., -300.5, -10.),
            Arc::new(material::Lambert {
                color: na::Vector3::new(0.5, 0.5, 0.5),
            }),
        )) as Arc<dyn shape::Shape>);

    let scene = Arc::new(scene::Scene { shapes: objects });
    let width: u32 = 800;
    let height: u32 = 600;
    let samples = 300;
    let film = film::Film::new(width, height, samples, Path::new("image.png").to_path_buf());
    let eye = na::Point3::new(0f32, 5f32, 15f32);
    let target = na::Point3::new(0f32, 0f32, 0f32);
    let view = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());
    let camera = Arc::new(camera::Camera::new(view.to_matrix(), 3.14 / 2.0, &film));
    let mut renderer = renderer::Renderer {
        film: film,
        camera: camera,
        scene: scene,
    };
    renderer.render();
}
