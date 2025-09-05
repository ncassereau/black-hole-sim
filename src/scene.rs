use macroquad::prelude::*;

use crate::BlackHole;
use crate::CartesianCoords3D;
use crate::Draw;
use crate::Ray;

pub struct Scene {
    scene_size: CartesianCoords3D,

    black_hole: BlackHole,
    rays: Vec<Ray>,

    dt: f64,
}

impl Scene {
    pub fn new(scene_width: f64, scene_height: f64, black_hole: BlackHole) -> Self {
        let scene_size = CartesianCoords3D::cartesian(scene_width, scene_height, 0.);
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
        let center: CartesianCoords3D = self.center_coords();
        let ratios: CartesianCoords3D = self.size_ratios();
        let result: CartesianCoords3D = center / ratios;
        let (center_x, center_y, _) = result.unpack_as_f32();
        draw_circle(center_x, center_y, 10., BLUE);
    }

    pub fn scene_size(&self) -> CartesianCoords3D {
        self.scene_size
    }

    pub fn screen_size(&self) -> CartesianCoords3D {
        CartesianCoords3D::cartesian(screen_width() as f64, screen_height() as f64, 0.)
    }

    pub fn center_coords(&self) -> CartesianCoords3D {
        self.scene_size() / 2.
    }

    pub fn size_ratios(&self) -> CartesianCoords3D {
        let screen_size = self.screen_size();
        self.scene_size / screen_size
    }

    pub fn min_size_ratio(&self) -> f64 {
        let (ratio_x, ratio_y, _) = self.size_ratios().unpack();
        ratio_x.min(ratio_y)
    }

    pub fn to_screen_coords(&self, coords: CartesianCoords3D) -> CartesianCoords3D {
        let (screen_width, screen_height, _) = self.screen_size().unpack();
        let (scene_width, scene_height, _) = self.scene_size().unpack();
        let uniform_ratio = self.min_size_ratio();

        let transformed = (coords + self.center_coords()) / uniform_ratio;

        // Center on screen
        let offset = CartesianCoords3D::cartesian(
            (screen_width - scene_width / uniform_ratio) / 2.0,
            (screen_height - scene_height / uniform_ratio) / 2.0,
            0.,
        );

        transformed + offset
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
