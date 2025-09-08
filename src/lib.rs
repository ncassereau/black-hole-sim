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
mod tensors;
mod threading;

pub use black_hole::BlackHole;
pub use constants::*;
pub use ray::Ray;
pub use scene::Scene;
pub use tensors::*;
pub use threading::*;

pub async fn launch() {
    let scene = Scene::new(
        crate::SCENE_WIDTH_FACTOR,
        crate::SCENE_HEIGHT_FACTOR,
        BlackHole::sagittarius(),
    );
    let sleep = Duration::from_millis(30);

    clear_background(BLACK);
    next_frame().await;

    let image = scene.get_image();
    let texture = Texture2D::from_image(&image);
    loop {
        let start = Instant::now();

        draw_texture(&texture, 0., 0., WHITE); // Last color is the Hue, we want None
        next_frame().await;
        let elapsed = start.elapsed();
        if sleep > elapsed {
            thread::sleep(sleep - elapsed);
        }
        println!("{}", start.elapsed().as_micros());
    }
}
