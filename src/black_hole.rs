use macroquad::prelude::*;
use ndarray::Array1;

use crate::Draw;
use crate::Scene;

pub struct BlackHole {
    coords: Array1<f64>,
    radius: f64,
    mass: f64,
}

impl BlackHole {
    pub fn sagittarius() -> Self {
        Self::new(0., 0., 8.6e36)
    }

    pub fn new(x: f64, y: f64, mass: f64) -> Self {
        let coords = Array1::from_vec(vec![x, y]);
        let radius = Self::compute_schwarzschild_radius(&mass);
        Self {
            coords,
            radius,
            mass,
        }
    }

    pub fn coords(&self) -> &Array1<f64> {
        &self.coords
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn compute_schwarzschild_radius(mass: &f64) -> f64 {
        2. * crate::GRAVITATIONAL_CONSTANT * mass / (crate::SPEED_OF_LIGHT * crate::SPEED_OF_LIGHT)
    }
}

impl Draw for BlackHole {
    fn draw(&self, scene: &Scene) {
        let centered = scene.to_screen_coords(&self.coords);
        let ratios = scene.size_ratios();
        self.draw_circle(centered, self.radius / ratios[0].min(ratios[1]), RED);
    }
}
