use macroquad::prelude::*;

use crate::CartesianCoords4D;
use crate::Draw;
use crate::Scene;

pub struct BlackHole {
    coords: CartesianCoords4D,
    radius: f64,
    mass: f64,
}

impl BlackHole {
    pub fn sagittarius() -> Self {
        Self::new(
            CartesianCoords4D::cartesian(0., 0., 0., 0.),
            4.15e6 * crate::SOLAR_MASS,
        )
    }

    pub fn new(coords: CartesianCoords4D, mass: f64) -> Self {
        let radius = Self::compute_schwarzschild_radius(&mass);
        Self {
            coords,
            radius,
            mass,
        }
    }

    pub fn coords(&self) -> &CartesianCoords4D {
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
        let centered = scene.to_screen_coords(self.coords);
        self.draw_circle(centered, self.radius / scene.min_size_ratio(), RED);
    }
}
