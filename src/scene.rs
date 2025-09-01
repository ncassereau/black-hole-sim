use macroquad::prelude::*;
use ndarray::Array1;

use crate::BlackHole;
use crate::Draw;
use crate::Ray;

pub struct Scene {
    scene_size: Array1<f64>,

    black_hole: BlackHole,
    rays: Vec<Ray>,

    dt: f64,
}

impl Scene {
    pub fn new(scene_width: f64, scene_height: f64, black_hole: BlackHole) -> Self {
        let scene_size = Array1::from_vec(vec![scene_width, scene_height]);
        let rays = Vec::new();
        Self {
            scene_size,
            black_hole,
            rays,
            dt: 1.,
        }
    }

    pub fn render(&self) {
        clear_background(BLACK);
        self.black_hole.draw(&self);

        for ray in &self.rays {
            ray.draw(&self);
        }
        let center = self.center_coords() / self.size_ratios();
        draw_circle(center[0] as f32, center[1] as f32, 10., BLUE);
    }

    pub fn scene_size(&self) -> &Array1<f64> {
        &self.scene_size
    }

    pub fn screen_size(&self) -> Array1<f64> {
        Array1::from_vec(vec![screen_width() as f64, screen_height() as f64])
    }

    pub fn center_coords(&self) -> Array1<f64> {
        &self.scene_size / 2.
    }

    pub fn size_ratios(&self) -> Array1<f64> {
        let screen_size = self.screen_size();
        &self.scene_size / &screen_size
    }

    pub fn to_screen_coords(&self, coords: &Array1<f64>) -> Array1<f64> {
        // let ratios = self.size_ratios();
        // (coords + self.center_coords()) / ratios[0].min(ratios[1])
        let screen_size = self.screen_size();
        let ratios = self.size_ratios();
        let uniform_ratio = ratios[0].min(ratios[1]);

        let transformed = (coords + self.center_coords()) / uniform_ratio;

        // Centrer sur l'écran
        let offset_x = (screen_size[0] - self.scene_size[0] / uniform_ratio) / 2.0;
        let offset_y = (screen_size[1] - self.scene_size[1] / uniform_ratio) / 2.0;

        transformed + Array1::from_vec(vec![offset_x, offset_y])
    }

    pub fn add_ray(&mut self, ray: Ray) {
        self.rays.push(ray);
    }

    pub fn step(&mut self) {
        let black_hole = &self.black_hole;
        for ray in &mut self.rays {
            ray.step(black_hole, self.dt);
        }
    }

    pub fn black_hole(&self) -> &BlackHole {
        &self.black_hole
    }
}
