use cudarc::driver::{CudaSlice, DeviceRepr};

use crate::{BlackHole, Hyperparameters, Scene, Skybox, black_hole::AccretionDisk, scene::Camera};

#[repr(C)]
pub struct CUDABlackHole {
    pub radius: f64,
    pub visual_radius: f64,
    pub color: [f32; 4],
}

impl From<&BlackHole> for CUDABlackHole {
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
unsafe impl DeviceRepr for CUDABlackHole {}

#[repr(C)]
pub struct CUDAAccretionDisk {
    pub r_isco: f64,
    pub accretion_r_max: f64,
    pub width: f64,
    pub max_temperature: f64,
    pub step_opacity: f64,
    pub doppler_factor: f64,
    pub fade_start_ratio: f64,
    pub peak_brigthness: f64,
}

impl From<&AccretionDisk> for CUDAAccretionDisk {
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

unsafe impl DeviceRepr for CUDAAccretionDisk {}

#[repr(C)]
pub struct CUDACamera {
    pub position: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub forward: [f32; 3],
    pub scale: f32,
    pub aspect_ratio: f32,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl CUDACamera {
    pub fn from_camera_scene(camera: &Camera, scene: &Scene) -> Self {
        let position = {
            let coords = camera.position().unpack_as_f32();
            [coords.0, coords.1, coords.2]
        };
        let right = {
            let coords = camera.right().unpack_as_f32();
            [coords.0, coords.1, coords.2]
        };
        let forward = {
            let coords = camera.forward().unpack_as_f32();
            [coords.0, coords.1, coords.2]
        };
        let up = {
            let coords = camera.up().unpack_as_f32();
            [coords.0, coords.1, coords.2]
        };
        let (screen_width, screen_height) = scene.screen_size().unpack_as_f32();

        Self {
            position,
            right,
            up,
            forward,
            scale: camera.scale() as f32,
            aspect_ratio: scene.aspect_ratio() as f32,
            screen_width: screen_width as u32,
            screen_height: screen_height as u32,
        }
    }
}

unsafe impl DeviceRepr for CUDACamera {}

#[repr(C)]
pub struct CUDAHyperparameters {
    pub dλ0: f64,
    pub bounding_box_radius: f64,
    pub num_integration_steps: u32,
    pub normalization_interval: u32,
    pub integration_error_tolerance: f64,
    pub min_dλ: f64,
    pub max_dλ: f64,
    pub max_dλ_ratio: f64,
    pub max_retries: u32,
}

impl CUDAHyperparameters {
    pub fn new(
        dλ0: f64,
        bounding_box_radius: f64,
        num_integration_steps: u32,
        normalization_interval: u32,
        integration_error_tolerance: f64,
        min_dλ: f64,
        max_dλ: f64,
        max_dλ_ratio: f64,
        max_retries: u32,
    ) -> Self {
        Self {
            dλ0,
            bounding_box_radius,
            num_integration_steps,
            normalization_interval,
            integration_error_tolerance,
            min_dλ,
            max_dλ,
            max_dλ_ratio,
            max_retries,
        }
    }
}

impl From<&Hyperparameters> for CUDAHyperparameters {
    fn from(value: &Hyperparameters) -> Self {
        Self::new(
            value.dλ0,
            value.bounding_box_radius,
            value.num_integration_steps as u32,
            value.normalization_interval as u32,
            value.integration_error_tolerance,
            value.min_dλ,
            value.max_dλ,
            value.max_dλ_ratio,
            value.max_retries as u32,
        )
    }
}

unsafe impl DeviceRepr for CUDAHyperparameters {}

#[repr(C)]
pub struct CUDASkybox {
    data: *mut f32,
    width: u32,
    height: u32,
}

impl CUDASkybox {
    pub fn new(data: *mut f32, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

unsafe impl DeviceRepr for CUDASkybox {}
