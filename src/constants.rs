use macroquad::color::{BLACK, Color};

pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
pub const DIV_EPSILON: f64 = 1e-8;
pub const CAMERA_THETA_EPSILON: f64 = 1e-3;
pub const CAMERA_ROTATION_SENSITIVITY: f64 = 1.;

pub const AU: f64 = 149_597_870_700.0; // meters

pub const SCENE_WIDTH_FACTOR: f64 = 200.;
pub const SCENE_HEIGHT_FACTOR: f64 = 200.;
pub const BOUNDING_BOX_FACTOR: f64 = 400.;
pub const BLACK_HOLE_COLORED_SPHERE_RADIUS_FACTOR: f64 = 1.01; // needs to be > 1;

pub const INTEGRATION_STEP_FACTOR: f64 = 0.1;
pub const NUM_INTEGRATION_STEPS: usize = 1000;
pub const NORMALIZATION_INTERVAL: usize = 10;

pub const RKF45_TOLERANCE_FACTOR: f64 = 1e-8;
pub const RKF45_MIN_STEP_FACTOR: f64 = 1e-4;
pub const RKF45_MAX_STEP_FACTOR: f64 = 1e2;
pub const RKF45_RETRIES: usize = 20;

pub const BACKGROUND_COLOR: Color = BLACK;

pub const NUM_THREADS: u32 = 24;
pub const FOV: f64 = 20.; // degrees

pub const SKYBOX_PATH: &str = "/workspace/hubble_skybox.tif";
