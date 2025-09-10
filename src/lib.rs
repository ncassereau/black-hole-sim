use std::{
    thread,
    time::{Duration, Instant},
};

use macroquad::prelude::*;

mod black_hole;
mod constants;
mod geodesic;
mod ray;
mod scene;
mod skybox;
mod tensors;
mod threading;

pub use black_hole::BlackHole;
pub use constants::*;
pub use ray::Ray;
pub use scene::Scene;
pub use skybox::*;
pub use tensors::*;
pub use threading::*;

pub async fn launch() {
    clear_background(BLACK);
    next_frame().await;

    let mut scene = Scene::new(
        crate::SCENE_WIDTH_FACTOR,
        crate::SCENE_HEIGHT_FACTOR,
        BlackHole::sagittarius(),
    );
    let sleep = Duration::from_millis(30);

    scene.rotate_camera(0., -5.);
    loop {
        let start = Instant::now();
        clear_background(BLACK);
        let image = scene.get_image();
        let texture = Texture2D::from_image(&image);
        draw_texture(&texture, 0., 0., WHITE); // Last color is the Hue, we want None

        next_frame().await;
        let elapsed = start.elapsed();
        if sleep > elapsed {
            thread::sleep(sleep - elapsed);
        }
        println!("{}", start.elapsed().as_micros());
        scene.rotate_camera(10., 0.);
    }
}
