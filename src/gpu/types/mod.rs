use bytemuck::{Pod, Zeroable};

use crate::{BlackHole, Scene, black_hole::AccretionDisk, scene::Camera};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GPUBlackHole {
    pub radius: f64,
    pub visual_radius: f64,
    pub color: [f32; 4],
}

impl From<&BlackHole> for GPUBlackHole {
    fn from(value: &BlackHole) -> Self {
        let color = value.color();
        let color = [color.r, color.g, color.b, color.a];
        Self {
            radius: value.radius(),
            visual_radius: value.visual_radius(),
            color,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GPUAccretionDisk {
    pub r_isco: f64,
    pub accretion_r_max: f64,
    pub width: f64,
    pub max_temperature: f64,
    pub step_opacity: f64,
    pub doppler_factor: f64,
    pub fade_start_ratio: f64,
    pub peak_brigthness: f64,
}

impl From<&AccretionDisk> for GPUAccretionDisk {
    fn from(value: &AccretionDisk) -> Self {
        Self {
            r_isco: value.r_isco(),
            accretion_r_max: value.accretion_r_max(),
            width: value.width(),
            step_opacity: value.step_opacity(),
            max_temperature: value.max_temperature(),
            doppler_factor: value.doppler_factor(),
            fade_start_ratio: value.fade_start_ratio(),
            peak_brigthness: value.peak_brigthness(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct GPUCamera {
    pub position: [f64; 3],
    pub _pad0: f64,
    pub right: [f64; 3],
    pub _pad1: f64,
    pub up: [f64; 3],
    pub _pad2: f64,
    pub forward: [f64; 3],
    pub _pad3: f64,
    pub scale: f64,
    pub aspect_ratio: f64,
    pub screen_width: f64,
    pub screen_height: f64,
}

impl GPUCamera {
    pub fn from_camera_scene(camera: &Camera, scene: &Scene) -> Self {
        let position = {
            let coords = camera.position().unpack();
            [coords.0, coords.1, coords.2]
        };
        let right = {
            let coords = camera.right().unpack();
            [coords.0, coords.1, coords.2]
        };
        let forward = {
            let coords = camera.forward().unpack();
            [coords.0, coords.1, coords.2]
        };
        let up = {
            let coords = camera.up().unpack();
            [coords.0, coords.1, coords.2]
        };
        let (screen_width, screen_height) = scene.screen_size().unpack();

        Self {
            position,
            right,
            up,
            forward,
            scale: camera.scale(),
            aspect_ratio: scene.aspect_ratio(),
            screen_width,
            screen_height,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct GPUHyperparameters {
    pub dλ0: f64,
    pub bounding_box_radius: f64,
    pub num_integration_steps: u64,
    pub normalization_interval: u64,
    // pub _pad0: f64,
    // pub _pad1: f64,
}

impl GPUHyperparameters {
    pub fn new(
        dλ0: f64,
        bounding_box_radius: f64,
        num_integration_steps: usize,
        normalization_interval: usize,
    ) -> Self {
        Self {
            dλ0,
            bounding_box_radius,
            num_integration_steps: num_integration_steps as u64,
            normalization_interval: normalization_interval as u64,
            ..Default::default()
        }
    }
}
