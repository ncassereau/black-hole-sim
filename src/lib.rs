use std::{thread, time::{Duration, Instant}};

use macroquad::prelude::*;

use ndarray;

mod black_hole;
mod draw;
mod ray;
mod scene;

pub use black_hole::BlackHole;
pub use scene::Scene;
pub use draw::Draw;
pub use ray::Ray;

const SCENE_WIDTH: f64 = 100_000_000_000.;
const SCENE_HEIGHT: f64 = 100_000_000_000.;



pub async fn launch() {
    
    request_new_screen_size(1920., 1080.);
    let mut scene = Scene::new(
        SCENE_WIDTH, SCENE_HEIGHT, BlackHole::sagittarius(),
    );
    let sleep = Duration::from_millis(15);

    for i in -5..5 {
        let ray = Ray::new(
            -SCENE_WIDTH / 2.,
            (i as f64 + 0.5) * 10_000_000_000.,
            1.,
            0.,
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