use macroquad::prelude::*;
use std::f64::consts::PI;

use crate::BlackHole;
use crate::Norm;
use crate::Ray;
use crate::SphericalCoords3D;
use crate::{CartesianCoords2D, CartesianCoords3D, CartesianCoords4D, CartesianState3D};

fn get_basis(
    position: CartesianCoords3D,
    target: CartesianCoords3D,
) -> (CartesianCoords3D, CartesianCoords3D, CartesianCoords3D) {
    let world_up = CartesianCoords3D::cartesian(0.0, 0.0, 1.0);
    let forward = (target - position).normalize();
    let right = forward.cross(world_up).normalize();
    let up = right.cross(forward).normalize();
    (forward, right, up)
}

fn get_pixel_color(
    camera: Camera,
    ray_direction: CartesianCoords3D,
    black_hole: BlackHole,
    dλ0: f64,
) -> Color {
    let camera_coords = camera.position();
    let ray_direction = camera.to_world_coordinates(ray_direction);
    let mut ray = Ray::new(
        CartesianState3D::cartesian(
            camera_coords.x(),
            camera_coords.y(),
            camera_coords.z(),
            ray_direction.x(),
            ray_direction.y(),
            ray_direction.z(),
        ),
        black_hole.radius(),
        dλ0,
    );
    let bounding_box_radius = black_hole.radius() * crate::BOUNDING_BOX_FACTOR;
    ray.get_color(black_hole, bounding_box_radius)
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    position: CartesianCoords3D,
    target: CartesianCoords3D,
    forward: CartesianCoords3D,
    up: CartesianCoords3D,
    right: CartesianCoords3D,
}

impl Camera {
    pub fn new(position: CartesianCoords3D, target: CartesianCoords3D) -> Self {
        let (forward, right, up) = get_basis(position, target);
        Self {
            position,
            target,
            forward,
            up,
            right,
        }
    }

    pub fn position(&self) -> CartesianCoords3D {
        self.position
    }

    pub fn rotate(&self, angle_x: f64, angle_y: f64) -> Self {
        let position = self.position.to_spherical();
        let theta = (position.theta() + angle_y).clamp(
            crate::CAMERA_THETA_EPSILON,
            PI - crate::CAMERA_THETA_EPSILON,
        );
        let phi = position.phi() + angle_x;
        let new_position = SphericalCoords3D::spherical(position.r(), theta, phi);
        Self::new(new_position.to_cartesian(), self.target)
    }

    pub fn to_world_coordinates(&self, direction: CartesianCoords3D) -> CartesianCoords3D {
        // At first the ray direction is given in coordinates relative to camera orientation.
        // Let's transform it to real world cartesian coordinates
        (self.right * direction.x() + self.up * direction.y() + self.forward * direction.z())
            .normalize()
    }
}

pub struct Scene {
    camera: Camera,
    scene_size: CartesianCoords2D,
    black_hole: BlackHole,
    dλ0: f64,
}

impl Scene {
    pub fn new(scene_width_factor: f64, scene_height_factor: f64, black_hole: BlackHole) -> Self {
        let radius = black_hole.radius();

        let scene_size =
            CartesianCoords2D::cartesian(scene_width_factor * radius, scene_height_factor * radius);
        let camera = Camera::new(
            CartesianCoords3D::cartesian(-scene_size.x() / 2., 0., 0.),
            black_hole.coords().position(),
        );
        Self {
            camera,
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

    pub fn camera(&self) -> Camera {
        self.camera
    }

    pub fn rotate_camera(&mut self, angle_x: f64, angle_y: f64) {
        self.camera = self.camera.rotate(
            angle_x.to_radians(),
            angle_y.to_radians(),
        );
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
                let ndc_y = 1.0 - 2.0 * (py as f64 + 0.5) / (screen_height as f64);
                let dλ0 = self.dλ0();
                let camera_clone = self.camera();
                let black_hole = self.black_hole();

                // Define the direction of the Ray
                // This is camera space!
                // Camera has the convention of looking towards the target so z coordinates in camera space has to be +1 (not -1).
                let ray_direction =
                    CartesianCoords3D::cartesian(ndc_x * scale * aspect_ratio, ndc_y * scale, 1.);

                pool.execute(move || {
                    (
                        px,
                        py,
                        get_pixel_color(camera_clone, ray_direction, black_hole, dλ0),
                    )
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
