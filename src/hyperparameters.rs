pub struct Hyperparameters {
    pub dλ0: f64,
    pub bounding_box_radius: f64,
    pub num_integration_steps: usize,
    pub normalization_interval: usize,
    pub integration_error_tolerance: f64,
    pub min_dλ: f64,
    pub max_dλ: f64,
    pub max_dλ_ratio: f64,
    pub max_retries: usize,
}

impl Hyperparameters {
    pub fn new(
        dλ0: f64,
        bounding_box_radius: f64,
        num_integration_steps: usize,
        normalization_interval: usize,
        integration_error_tolerance: f64,
        min_dλ: f64,
        max_dλ: f64,
        max_dλ_ratio: f64,
        max_retries: usize,
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
