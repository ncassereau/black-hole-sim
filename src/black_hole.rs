use macroquad::prelude::*;

use crate::{CartesianCoords3D, CartesianCoords4D};

fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}

#[derive(Debug, Clone, Copy)]
pub struct AccretionDisk {
    r_isco: f64,
    accretion_r_max: f64,
    width: f64,
    max_temperature: f64,
    step_opacity: f64,
    doppler_factor: f64,
    fade_start_ratio: f64, // At which radius do we start to fade out
    peak_brigthness: f64,  // highest brightness
}

impl AccretionDisk {
    pub fn new(rs: f64) -> Self {
        let r_isco = rs * 3.;
        let accretion_r_max = rs * 15.;
        let r_peak = r_isco * (49.0 / 36.0);
        Self {
            r_isco,
            accretion_r_max,
            width: accretion_r_max - r_isco,
            max_temperature: 18000.0,
            step_opacity: 0.04,
            doppler_factor: 3.0,
            fade_start_ratio: 0.5,
            peak_brigthness: Self::brightness(r_isco, r_peak),
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

    fn kelvin_to_rgb(temp_kelvin: f64) -> (f64, f64, f64) {
        let temp = temp_kelvin / 100.0;
        let red = if temp <= 66.0 {
            255.0
        } else {
            329.698727446 * (temp - 60.0).powf(-0.1332047592)
        };
        let green = if temp <= 66.0 {
            99.4708025861 * temp.ln() - 161.1195681661
        } else {
            288.1221695283 * (temp - 60.0).powf(-0.0755148492)
        };
        let blue = if temp <= 19.0 {
            0.0
        } else if temp >= 66.0 {
            255.0
        } else {
            138.5177312231 * (temp - 10.0).ln() - 305.0447927307
        };
        (
            (red.clamp(0., 255.)) / 255.0,
            (green.clamp(0., 255.)) / 255.0,
            (blue.clamp(0., 255.)) / 255.0,
        )
    }

    fn brightness(r_isco: f64, radius: f64) -> f64 {
        (1.0 - (r_isco / radius).sqrt()) / (radius.powi(3) + crate::DIV_EPSILON)
    }

    pub fn get_color(&self, radius: f64) -> Option<Color> {
        // x_pos for Doppler effect
        if radius < self.r_isco || radius > self.accretion_r_max {
            return None;
        }

        // Shakura-Sunyaev
        let normalized_brightness =
            (Self::brightness(self.r_isco, radius) / self.peak_brigthness).min(1.0);

        let normalized_radius = (radius - self.r_isco) / self.width;
        let geometric_falloff = 1.0 - smoothstep(self.fade_start_ratio, 1.0, normalized_radius);

        // Stefan-Boltzmann's Law
        let temp_k = self.max_temperature * normalized_brightness.powf(0.25);
        // Planck's Law
        let (r, g, b) = Self::kelvin_to_rgb(temp_k);

        // Doppler effect for later
        // let doppler_boost = (1.0 - x_pos / r_out).powf(doppler_factor);
        let doppler_boost = 1.0; // Disabled for now

        let emitted_intensity = normalized_brightness * doppler_boost * geometric_falloff;

        let emitted_r = r * emitted_intensity;
        let emitted_g = g * emitted_intensity;
        let emitted_b = b * emitted_intensity;
        let opacity_for_step = self.step_opacity * emitted_intensity;

        Some(Color::new(
            emitted_r as f32,
            emitted_g as f32,
            emitted_b as f32,
            opacity_for_step as f32,
        ))
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
            color: BLACK,
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
