use ndarray::Array1;
use macroquad::prelude::*;

use crate::Draw;
use crate::Scene;

pub struct BlackHole {
    coords: Array1<f64>,
    radius: f64,
    mass: f64,
}


impl BlackHole {

    pub fn sagittarius() -> Self {
        Self::new(
            0.,
            0.,
            12_000_000_000.,
            8.6 * num::pow(10.0, 36),
        )
    }

    pub fn new(x: f64, y: f64, radius: f64, mass: f64) -> Self {
        let coords = Array1::from_vec(vec![x, y]);
        Self {
            coords,
            radius,
            mass,
        }
    }


}

impl Draw for BlackHole {
    
    fn draw(&self, scene: &Scene) {
        let centered = scene.to_screen_coords(&self.coords);
        self.draw_circle(
            centered,
            self.radius / scene.size_ratios()[1],
            RED,
        );
    }

}