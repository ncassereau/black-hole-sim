use macroquad::color::Color;
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line};

use crate::CartesianCoords4D;
use crate::scene::Scene;

pub trait Draw {
    fn draw(&self, scene: &Scene);

    fn draw_circle(&self, coords: CartesianCoords4D, radius: f64, color: Color) {
        let (x, y) = coords.unpack_xy_as_f32();
        let radius = radius as f32;
        draw_circle(x, y, radius - 1., color);
        draw_circle_lines(x, y, radius - 3., 2., color);
    }

    fn draw_line(
        &self,
        coords1: CartesianCoords4D,
        coords2: CartesianCoords4D,
        thickness: f32,
        color: Color,
    ) {
        let (x1, y1) = coords1.unpack_xy_as_f32();
        let (x2, y2) = coords2.unpack_xy_as_f32();
        draw_line(x1, y1, x2, y2, thickness, color);
    }
}
