use macroquad::prelude::*;

use crate::{CartesianCoords3D, CartesianCoords4D};

#[derive(Debug, Clone, Copy)]
pub struct AccretionDisk {
    r_isco: f64,
    accretion_r_max: f64,
}

impl AccretionDisk {
    pub fn new(rs: f64) -> Self {
        let r_isco = rs * 3.;
        let accretion_r_max = rs * 12.;
        Self {
            r_isco,
            accretion_r_max,
        }
    }

    pub fn check_intersection(
        &self,
        position1: CartesianCoords3D,
        position2: CartesianCoords3D,
    ) -> Option<f64> {
        let t = -position1.z() / (position2.z() - position1.z());
        if t < 0. || t > 1. {
            return None;
        }
        let equator_collision = position1 + (position2 - position1) * t;
        let r_plane = equator_collision.to_spherical().r();
        if r_plane < self.r_isco || r_plane > self.accretion_r_max {
            return None;
        }

        Some(r_plane)
    }

    pub fn get_color(&self, radius: f64) -> Option<Color> {
        if radius >= self.r_isco && radius < self.accretion_r_max {
            let u = (radius - self.r_isco) / (self.accretion_r_max - self.r_isco);
            let brightness = (1.0 - u).powf(2.0);
            Some(Color::new(
                1.0 * brightness as f32,
                0.6 * brightness as f32,
                0.0,
                1.0,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BlackHole {
    coords: CartesianCoords4D,
    radius: f64,
    visual_radius: f64,
    accretion_disk: AccretionDisk,
    mass: f64,
    color: Color,
}

impl BlackHole {
    pub fn sagittarius() -> Self {
        Self::new(CartesianCoords4D::cartesian(0., 0., 0., 0.), 1.)
    }

    pub fn new(coords: CartesianCoords4D, mass: f64) -> Self {
        let radius = Self::compute_schwarzschild_radius(&mass);
        let accretion_disk = AccretionDisk::new(radius);
        let visual_radius = radius * crate::BLACK_HOLE_COLORED_SPHERE_RADIUS_FACTOR;
        Self {
            coords,
            radius,
            visual_radius,
            accretion_disk,
            mass,
            color: WHITE,
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

    pub fn accretion_disk(&self) -> AccretionDisk {
        self.accretion_disk
    }

    pub fn visual_radius(&self) -> f64 {
        self.visual_radius
    }

    pub fn color(&self) -> Color {
        self.color
    }

    fn compute_schwarzschild_radius(mass: &f64) -> f64 {
        // In normalised units (G = c = 1)
        2. * mass
    }
}
