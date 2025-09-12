
struct GPUHyperparameters {
    dλ0: f64,
    bounding_box_radius: f64,
    num_integration_steps: u32,
    normalization_interval: u32,
}

struct GPUBlackHole {
    radius: f64,
    visual_radius: f64,
    color: GPUColor,
}

struct GPUCamera {
    position: vec4<f64>,
    right: vec4<f64>,
    up: vec4<f64>,
    forward: vec4<f64>,
    scale: f64, // This is the tangent of FOV
    aspect_ratio: f64, // Width / height
    screen_width: f64,
    screen_height: f64,
}

