use macroquad::prelude::*;
use ndarray::Array1;

use crate::Draw;
use crate::BlackHole;
use crate::Ray;

pub struct Scene {

    scene_size: Array1<f64>,

    black_hole: BlackHole,
    rays: Vec<Ray>,

}

impl Scene {

    pub fn new(
        scene_width: f64,
        scene_height: f64,
        black_hole: BlackHole,
    ) -> Self {
        let scene_size = Array1::from_vec(vec![scene_width, scene_height]);
        let rays = Vec::new();
        Self {
            scene_size,
            black_hole,
            rays,
        }
    }

    pub fn render(&self) {
        clear_background(BLACK);
        self.black_hole.draw(&self);

        for ray in &self.rays {
            ray.draw(&self);
        }
    }

    pub fn scene_size(&self) -> &Array1<f64> {
        &self.scene_size
    }

    pub fn screen_size(&self) -> Array1<f64> {
        Array1::from_vec(vec![
            screen_width() as f64,
            screen_height() as f64,
        ])
    }

    pub fn center_coords(&self) -> Array1<f64> {
        &self.scene_size / 2.
    }

    pub fn size_ratios(&self) -> Array1<f64> {
        let screen_size = self.screen_size();
        &self.scene_size / &screen_size
    }

    pub fn to_screen_coords(&self, coords: &Array1<f64>) -> Array1<f64> {
        (coords + self.center_coords()) / self.size_ratios()
    }

    pub fn add_ray(&mut self, ray: Ray) {
        self.rays.push(ray);
    }

    pub fn step(&mut self) {
        for ray in &mut self.rays {
            ray.step();
        }
    }

}

