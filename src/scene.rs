use macroquad::prelude::*;

use crate::BlackHole;
use crate::Draw;
use crate::Ray;
use crate::{CartesianCoords2D, CartesianCoords4D};

pub struct Scene {
    scene_size: CartesianCoords2D,

    black_hole: BlackHole,
    rays: Vec<Ray>,

    dλ: f64,
}

impl Scene {
    pub fn new(scene_width: f64, scene_height: f64, black_hole: BlackHole) -> Self {
        let scene_size = CartesianCoords2D::cartesian(scene_width, scene_height);
        let rays = Vec::new();
        Self {
            scene_size,
            black_hole,
            rays,
            dλ: 1.,
        }
    }

    pub fn render(&self) {
        clear_background(BLACK);
        self.black_hole.draw(&self);

        for ray in &self.rays {
            ray.draw(&self);
        }
        let center = self.center_coords();
        let ratios = self.size_ratios();
        let result = center / ratios;
        let (center_x, center_y) = result.unpack_as_f32();
        draw_circle(center_x, center_y, 10., BLUE);
    }

    pub fn scene_size(&self) -> CartesianCoords2D {
        self.scene_size
    }

    pub fn screen_size(&self) -> CartesianCoords2D {
        CartesianCoords2D::cartesian(screen_width() as f64, screen_height() as f64)
    }

    pub fn center_coords(&self) -> CartesianCoords2D {
        self.scene_size() / 2.
    }

    pub fn size_ratios(&self) -> CartesianCoords2D {
        let screen_size = self.screen_size();
        self.scene_size / screen_size
    }

    pub fn min_size_ratio(&self) -> f64 {
        let (ratio_x, ratio_y) = self.size_ratios().unpack();
        ratio_x.min(ratio_y)
    }

    pub fn to_screen_coords(&self, coords: CartesianCoords4D) -> CartesianCoords4D {
        let (screen_width, screen_height) = self.screen_size().unpack();
        let (scene_width, scene_height) = self.scene_size().unpack();
        let uniform_ratio = self.min_size_ratio();

        let transformed = (coords + self.center_coords()) / uniform_ratio;

        // Center on screen
        let offset = CartesianCoords2D::cartesian(
            (screen_width - scene_width / uniform_ratio) / 2.0,
            (screen_height - scene_height / uniform_ratio) / 2.0,
        );

        transformed + offset
    }

    pub fn add_ray(&mut self, ray: Ray) {
        self.rays.push(ray);
    }

    pub fn step(&mut self) {
        let black_hole = &self.black_hole;
        for ray in &mut self.rays {
            ray.step(black_hole, self.dλ);
        }
    }

    pub fn black_hole(&self) -> &BlackHole {
        &self.black_hole
    }
}
