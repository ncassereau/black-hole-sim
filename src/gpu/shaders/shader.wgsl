
@group(0) @binding(0) var<uniform> black_hole: GPUBlackHole;
@group(0) @binding(1) var<uniform> camera: GPUCamera;
@group(0) @binding(2) var<uniform> accretion_disk: GPUAccretionDisk;
@group(0) @binding(3) var<uniform> hyperparams: GPUHyperparameters;
@group(0) @binding(4) var<storage, read_write> output: array<GPUColor>;

@compute @workgroup_size(64)
fn compute_image(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    let num_pixels = u32(camera.screen_height * camera.screen_width);
    if (global_id.x >= num_pixels) {
        return;
    }
    let px = global_id.x % u32(camera.screen_width);
    let py = global_id.x / u32(camera.screen_width);

    let ndc_x = (f64(px) + 0.5) / camera.screen_width * 2.0 - 1.0;
    let ndc_y = 1.0 - 2.0 * (f64(py) + 0.5) / camera.screen_height;

    let ray = get_ray(ndc_x, ndc_y, camera, black_hole.radius);

    // output[global_id.x] = get_ray_color(
    //     ray, black_hole, accretion_disk, hyperparams
    // );
    output[global_id.x] = GPUColor(1.0, 0.0, 0.0, 1.0);
}