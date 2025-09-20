use std::{
    thread,
    time::{Duration, Instant},
};

use macroquad::prelude::*;

mod backend;
mod black_hole;
mod constants;
mod cuda;
mod geodesic;
mod hyperparameters;
mod ray;
mod scene;
mod skybox;
mod tensors;
mod threading;

pub use backend::Backend;
pub use black_hole::BlackHole;
pub use constants::*;
pub use cuda::*;
pub use hyperparameters::Hyperparameters;
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
    scene.rotate_camera(0., -5.);

    let hyperparams = Hyperparameters::new(
        scene.dλ0(),
        scene.black_hole().radius() * crate::BOUNDING_BOX_FACTOR,
        crate::NUM_INTEGRATION_STEPS,
        crate::NORMALIZATION_INTERVAL,
        scene.black_hole().radius() * crate::RKF45_TOLERANCE_FACTOR,
        scene.black_hole().radius() * crate::RKF45_MIN_STEP_FACTOR,
        scene.black_hole().radius() * crate::RKF45_MAX_STEP_FACTOR,
        crate::RKF45_MAX_STEP_RATIO,
        crate::RKF45_RETRIES,
    );

    let backend = CUDABackend::new().await.unwrap_or_else(|e| panic!("{e}"));

    clear_background(BLACK);
    next_frame().await;
    let start = Instant::now();
    let image = backend.compute(
        &scene.black_hole().accretion_disk(),
        &scene.black_hole(),
        &scene.camera(),
        &scene,
        &hyperparams,
    );
    println!("{}", start.elapsed().as_micros());

    loop {
        if let Ok(im) = &image {
            let texture = Texture2D::from_image(im);
            draw_texture(&texture, 0., 0., WHITE);
        }
        next_frame().await;
    }

    // clear_background(BLACK);
    // next_frame().await;

    // let mut scene = Scene::new(
    //     crate::SCENE_WIDTH_FACTOR,
    //     crate::SCENE_HEIGHT_FACTOR,
    //     BlackHole::sagittarius(),
    // );
    // let sleep = Duration::from_millis(30);

    // scene.rotate_camera(0., -5.);
    // loop {
    //     let start = Instant::now();
    //     clear_background(BLACK);
    //     let image = scene.get_image();
    //     let texture = Texture2D::from_image(&image);
    //     draw_texture(&texture, 0., 0., WHITE); // Last color is the Hue, we want None

    //     next_frame().await;
    //     let elapsed = start.elapsed();
    //     if sleep > elapsed {
    //         thread::sleep(sleep - elapsed);
    //     }
    //     println!("{}", start.elapsed().as_micros());
    //     scene.rotate_camera(1., 0.);
    // }
}
