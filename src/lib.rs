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
mod vectors;

pub use black_hole::BlackHole;
pub use constants::*;
pub use draw::Draw;
pub use ray::Ray;
pub use scene::Scene;
pub use vectors::*;

pub async fn launch() {
    request_new_screen_size(1920., 1080.);
    let mut scene = Scene::new(
        crate::SCENE_WIDTH,
        crate::SCENE_HEIGHT,
        BlackHole::sagittarius(),
    );
    let sleep = Duration::from_millis(15);

    for i in -20..20 {
        let ray = Ray::new(
            -SCENE_WIDTH / 2.,
            (i as f64 + 0.5) * 3_000_000_000.,
            0.,
            SPEED_OF_LIGHT,
            0.,
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
