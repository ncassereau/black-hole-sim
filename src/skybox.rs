use crate::{CartesianCoords3D, Norm};
use image::DynamicImage;
use macroquad::color::Color;
use macroquad::rand::gen_range;
use std::error::Error;

pub struct Skybox {
    data: Vec<[f32; 3]>,
    width: u32,
    height: u32,
}

fn load_skybox(path: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let img = image::open(path)?;
    Ok(img)
}

impl Skybox {
    pub fn from_path(path: &str) -> Self {
        let img = load_skybox(path).unwrap_or_else(|e| panic!("{e}"));
        Self::from_image(img)
    }

    pub fn as_f32_slice(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const f32, self.data.len() * 3) }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn generate_star_field(width: u32, height: u32) -> Self {
        let mut data = vec![[0.0, 0.0, 0.0]; (width * height) as usize];

        for _ in 0..100000 {
            let cx = gen_range(3, width - 3) as f32;
            let cy = gen_range(3, height - 3) as f32;

            let size_roll = gen_range(0.0, 1.0);
            let star_size: f32 = if size_roll < 0.7 {
                gen_range(0.1, 0.5)
            } else if size_roll < 0.9 {
                gen_range(0.5, 1.0)
            } else {
                gen_range(1.0, 1.5)
            };

            let brightness = gen_range(0.2, 1.0) * (star_size / 4.0).min(1.0);

            let color_temp = gen_range(0.0, 1.0);
            let star_color = if color_temp < 0.3 {
                [brightness, brightness * 0.7, brightness * 0.4]
            } else if color_temp > 0.7 {
                [brightness * 0.7, brightness * 0.85, brightness]
            } else {
                [brightness; 3]
            };

            let max_radius = star_size.ceil() as i32 + 1;
            for dy in -max_radius..=max_radius {
                for dx in -max_radius..=max_radius {
                    let x = cx as i32 + dx;
                    let y = cy as i32 + dy;

                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        let distance = ((dx * dx + dy * dy) as f32).sqrt();
                        if distance <= star_size {
                            let intensity =
                                (-distance * distance * 5.0 / (star_size * star_size)).exp();
                            let idx = (y as u32 * width + x as u32) as usize;

                            data[idx][0] = (data[idx][0] + star_color[0] * intensity).min(1.0);
                            data[idx][1] = (data[idx][1] + star_color[1] * intensity).min(1.0);
                            data[idx][2] = (data[idx][2] + star_color[2] * intensity).min(1.0);
                        }
                    }
                }
            }
        }

        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_image(img: DynamicImage) -> Self {
        let rgb_img = img.to_rgb32f();
        let (width, height) = rgb_img.dimensions();

        if let Some(first_pixel) = rgb_img.pixels().next() {
            println!("Premier pixel: {:?}", first_pixel.0);
        }

        let data: Vec<[f32; 3]> = rgb_img
            .pixels()
            .map(|p| [p.0[0].powf(2.2), p.0[1].powf(2.2), p.0[2].powf(2.2)])
            .collect();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn sample(&self, direction: &CartesianCoords3D) -> Color {
        let direction = direction.normalize();
        let u = 0.5 + direction.z().atan2(direction.x()) as f32 / (2.0 * std::f32::consts::PI);
        let v = direction.y().acos() as f32 / std::f32::consts::PI;

        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);

        let index = (y * self.width + x) as usize;
        let [r, g, b] = self.data[index];
        Color { r, g, b, a: 1. }
    }
}
