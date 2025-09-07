pub const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s
pub const SQUARED_SPEED_OF_LIGHT: f64 = SPEED_OF_LIGHT * SPEED_OF_LIGHT;
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
pub const DIV_EPSILON: f64 = 1e-14;

pub const AU: f64 = 149_597_870_700.0; // meters
pub const SOLAR_MASS: f64 = 1.989e30; // kg

pub const SCENE_WIDTH_FACTOR: f64 = 100.;
pub const SCENE_HEIGHT_FACTOR: f64 = 100.;

pub const INTEGRATION_STEP_FACTOR: f64 = 1. / 10.;
pub const NUM_INTEGRATION_STEPS: usize = 20_000;

pub const RKF45_TOLERANCE_FACTOR: f64 = 1e-7;
pub const RKF45_MIN_STEP_FACTOR: f64 = 1e-9;
pub const RFK45_RETRIES: usize = 20;
