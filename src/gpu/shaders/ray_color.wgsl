const ENTERED_EVENT_HORIZON: u32 = 0u;
const OUT_OF_BOUNDING_BOX: u32 = 1u;
const CROSSED_ACCRETION_DISK: u32 = 2u;
const NO_STOPPING: u32 = 3u;

// Integration failures
const MIN_STEP_REACHED: u32 = 4u;
const MAX_RETRIES_REACHED: u32 = 5u;

struct StoppingResult {
    criterion: u32,
    direction: vec3<f64>,
    radius: f64,
}

struct StepResult {
    state: GPURay,
    dl: f64,
    stopping_result: StoppingResult,
}

fn determine_color(
    stopping_result: StoppingResult,
    black_hole: GPUBlackHole,
    accretion_disk: GPUAccretionDisk
) -> GPUColor {
    switch stopping_result.criterion {
        case ENTERED_EVENT_HORIZON: {
            return black_hole.color;
        }
        case OUT_OF_BOUNDING_BOX: {
            return get_skybox_color(stopping_result.direction);
        }
        case CROSSED_ACCRETION_DISK: {
            return get_accretion_disk_color(accretion_disk, stopping_result.radius);
        }
        default: {
            return GPUColor(0.0, 0.0, 0.0, 0.0);
        }
    }
}

fn step_ray(
    ray: GPURay,
    black_hole: GPUBlackHole,
    accretion_disk: GPUAccretionDisk,
    hyperparams: GPUHyperparameters,
    dl: f64
) -> StepResult {
    
    let current_r = ray.position[0];
    
    // Check event horizon
    if current_r <= black_hole.visual_radius {
        return StepResult(
            ray,
            dl,
            StoppingResult(ENTERED_EVENT_HORIZON, vec3<f64>(0.0), 0.0)
        );
    }
    
    // Solve geodesic step
    let rk_result = runge_kutta_fehlberg_45(
        ray,
        dl,
        hyperparams.integration_error_tolerance,
        black_hole.radius,
        hyperparams.min_dλ,
        hyperparams.max_dλ,
        hyperparams.max_retries,
    );

    if rk_result.return_code != NO_STOPPING {
        if current_r <= black_hole.visual_radius {
            return StepResult(
                rk_result.state,
                rk_result.new_h,
                StoppingResult(ENTERED_EVENT_HORIZON, vec3<f64>(0.0), 0.0)
            );
        } else {
            return StepResult(
                rk_result.state,
                rk_result.new_h,
                StoppingResult(OUT_OF_BOUNDING_BOX, vec3<f64>(0.0), 0.0)
            );
        }
    }
    
    // Check accretion disk intersection
    let disk_intersection = check_accretion_intersection(
        accretion_disk, ray.position.xyz, rk_result.state.position.xyz
    );
    if disk_intersection != -1 {
        return StepResult(
            rk_result.state,
            rk_result.new_h,
            StoppingResult(CROSSED_ACCRETION_DISK, vec3<f64>(0.0), disk_intersection)
        );
    }
    
    let new_r = rk_result.state.position[0];
    
    // Check bounding box + moving away
    if new_r > hyperparams.bounding_box_radius {
        let spatial_pos = rk_result.state.position.xyz;
        let spatial_vel = rk_result.state.direction.xyz;
        if dot(spatial_pos, spatial_vel) > 0.0 {
            return StepResult(
                rk_result.state,
                rk_result.new_h,
                StoppingResult(OUT_OF_BOUNDING_BOX, spatial_pos, 0.0)
            );
        }
    }
    
    return StepResult(
        rk_result.state,
        rk_result.new_h,
        StoppingResult(NO_STOPPING, vec3<f64>(0.0), 0.0),
    );
}

fn get_ray_color(
    ray: GPURay,
    black_hole: GPUBlackHole,
    accretion_disk: GPUAccretionDisk,
    hyperparams: GPUHyperparameters,
) -> GPUColor {
    var accumulated_color = GPUColorState(GPUColor(0., 0., 0., 0.), 1.0);
    var dl = hyperparams.dλ0;
    var current_ray = ray;

    for (var i = 0u; i < hyperparams.num_integration_steps; i++) {
        if i > 0 && i % hyperparams.normalization_interval == 0 {
            current_ray.direction = normalize_direction(
                current_ray.position, current_ray.direction, black_hole.radius
            );
        }

        let result = step_ray(current_ray, black_hole, accretion_disk, hyperparams, dl);
        current_ray = result.state;
        dl = result.dl;

        if result.stopping_result.criterion != NO_STOPPING {
            let hit_color = determine_color(
                result.stopping_result, black_hole, accretion_disk
            );
            accumulated_color = blend_color(accumulated_color, hit_color);
            if accumulated_color.transmittance < 0.05 {
                break;
            }
        }
    }

    accumulated_color = blend_color(accumulated_color, GPUColor(0., 0., 0., 0.));
    return gamma_correct(accumulated_color.color);
}
