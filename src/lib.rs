use std::{
    thread,
    time::{Duration, Instant},
};

use macroquad::prelude::*;

mod black_hole;
mod constants;
mod draw;
mod geodesic;
mod ray;
mod scene;
mod tensors;

pub use black_hole::BlackHole;
pub use constants::*;
pub use draw::Draw;
pub use ray::Ray;
pub use scene::Scene;
pub use tensors::*;

pub async fn launch() {
    request_new_screen_size(1920., 1080.);
    let mut scene = Scene::new(
        crate::SCENE_WIDTH_FACTOR,
        crate::SCENE_HEIGHT_FACTOR,
        BlackHole::sagittarius(),
    );
    let sleep = Duration::from_millis(15);

    let n_rays = 100;
    let (scene_width, scene_height) = scene.scene_size().unpack();
    let x0 = -scene_width / 2.;
    for i in -n_rays..n_rays {
        let ray = Ray::new(
            CartesianState3D::cartesian(
                x0,
                i as f64 * scene_height / n_rays as f64,
                0.,
                1.,
                0.,
                0.,
            ),
            scene.black_hole().coords(),
            scene.black_hole().radius(),
            scene.dλ0(),
        );
        scene.add_ray(ray);
    }

    loop {
        let start = Instant::now();
        scene.render();

        scene.step();

        next_frame().await;
        let elapsed = start.elapsed();
        if sleep > elapsed {
            thread::sleep(sleep - elapsed);
        }
        println!("{}", start.elapsed().as_micros());
    }
}
