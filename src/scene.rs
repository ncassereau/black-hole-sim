use macroquad::prelude::*;

use crate::BlackHole;
use crate::Norm;
use crate::Ray;
use crate::black_hole;
use crate::{CartesianCoords2D, CartesianCoords3D, CartesianCoords4D, CartesianState3D};

pub struct Scene {
    camera_coords: CartesianCoords3D,
    scene_size: CartesianCoords2D,

    black_hole: BlackHole,

    dλ0: f64,
}

impl Scene {
    pub fn new(scene_width_factor: f64, scene_height_factor: f64, black_hole: BlackHole) -> Self {
        let radius = black_hole.radius();

        let scene_size =
            CartesianCoords2D::cartesian(scene_width_factor * radius, scene_height_factor * radius);
        let camera_coords = CartesianCoords3D::cartesian(-scene_size.x() / 2., 0., 0.);
        Self {
            camera_coords,
            scene_size,
            black_hole,
            dλ0: radius * crate::INTEGRATION_STEP_FACTOR,
        }
    }

    pub fn dλ0(&self) -> f64 {
        self.dλ0
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

    pub fn black_hole(&self) -> BlackHole {
        self.black_hole
    }

    pub fn get_image(&self) -> Image {
        let (screen_width, screen_height) = self.screen_size().unpack();

        let aspect_ratio = screen_width / screen_height;
        let scale = (f64::to_radians(crate::FOV) / 2.0).tan();

        let num_pixels = (screen_width * screen_height) as u32;
        let mut counter: u32 = 0;

        let mut pool = crate::ThreadPool::new(crate::NUM_THREADS);

        for px in 0..screen_width as u32 {
            let ndc_x = (px as f64 + 0.5) / (screen_width as f64) * 2.0 - 1.0;
            for py in 0..screen_height as u32 {
                let ndc_y = (py as f64 + 0.5) / (screen_height as f64) * 2.0 - 1.0;
                let camera_clone = self.camera_coords.clone();
                let black_hole = self.black_hole();
                let ray_direction =
                    CartesianCoords3D::cartesian(1., ndc_y * scale, ndc_x * scale * aspect_ratio)
                        .normalize();
                let dλ0 = self.dλ0();

                pool.execute(move || {
                    let mut ray = Ray::new(
                        CartesianState3D::cartesian(
                            camera_clone.x(),
                            camera_clone.y(),
                            camera_clone.z(),
                            ray_direction.x(),
                            ray_direction.y(),
                            ray_direction.z(),
                        ),
                        black_hole.radius(),
                        dλ0,
                    );
                    (px, py, ray.get_color(black_hole))
                });

                counter += 1;
                if counter % 10_000 == 0 {
                    println!("Submitted {counter} / {num_pixels}");
                }
            }
        }

        pool.gather(screen_width as u16, screen_height as u16)
    }
}
