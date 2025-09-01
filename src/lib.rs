use std::{
    thread,
    time::{Duration, Instant},
};

use macroquad::prelude::*;

mod black_hole;
mod draw;
mod geodesic;
mod ray;
mod scene;

pub use black_hole::BlackHole;
pub use draw::Draw;
pub use ray::Ray;
pub use scene::Scene;

pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;
pub const SQUARED_SPEED_OF_LIGHT: f64 = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

const SCENE_WIDTH: f64 = 150_000_000_000.;
const SCENE_HEIGHT: f64 = 150_000_000_000.;

pub async fn launch() {
    request_new_screen_size(1920., 1080.);
    let mut scene = Scene::new(SCENE_WIDTH, SCENE_HEIGHT, BlackHole::sagittarius());
    let sleep = Duration::from_millis(15);

    for i in -20..20 {
        let ray = Ray::new(
            -SCENE_WIDTH / 2.,
            (i as f64 + 0.5) * 3_000_000_000.,
            SPEED_OF_LIGHT,
            0.,
            scene.black_hole().coords(),
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
        println!("{}", start.elapsed().as_millis());
    }
}
