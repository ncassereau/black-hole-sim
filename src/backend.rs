use macroquad::texture::Image;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;

use crate::{BlackHole, Hyperparameters, Scene, Skybox, black_hole::AccretionDisk, scene::Camera};

pub trait Backend: Sized {
    fn new() -> impl Future<Output = Result<Self, Box<dyn Error>>> + Send;

    fn compute(
        &mut self,
        accretion_disk: &AccretionDisk,
        black_hole: &BlackHole,
        skybox: Arc<Skybox>,
        camera: &Camera,
        scene: &Scene,
        hyperparams: &Hyperparameters,
    ) -> Result<Image, Box<dyn Error>>;

    fn to_image(v: Vec<f32>, width: u16, height: u16) -> Result<Image, Box<dyn Error>> {
        let mut image = Image::gen_image_color(width, height, macroquad::color::BLACK);

        for (index, chunk) in v.chunks(3).enumerate() {
            if chunk.len() != 3 {
                return Err(format!("Chunk size is {} instead of 3", chunk.len()).into());
            }
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let px = index as u32 % width as u32;
            let py = index as u32 / width as u32;

            if py < height as u32 {
                image.set_pixel(px, py, macroquad::color::Color { r, g, b, a: 1.0 });
            }
        }
        Ok(image)
    }
}
