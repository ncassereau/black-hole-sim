use macroquad::color::Color;
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line};
use ndarray::Array1;

use crate::scene::Scene;

pub trait Draw {
    fn draw(&self, scene: &Scene);

    fn draw_circle(&self, coords: Array1<f64>, radius: f64, color: Color) {
        draw_circle(
            coords[0] as f32,
            coords[1] as f32,
            (radius - 1.) as f32,
            color,
        );
        draw_circle_lines(
            coords[0] as f32,
            coords[1] as f32,
            (radius - 3.) as f32,
            2.,
            color,
        );
    }

    fn draw_line(
        &self,
        coords1: &Array1<f64>,
        coords2: &Array1<f64>,
        thickness: f32,
        color: Color,
    ) {
        draw_line(
            coords1[0] as f32,
            coords1[1] as f32,
            coords2[0] as f32,
            coords2[1] as f32,
            thickness,
            color,
        );
    }
}
