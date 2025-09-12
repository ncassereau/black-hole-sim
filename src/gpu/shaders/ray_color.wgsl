// Enum simulé avec des constantes
const ENTERED_EVENT_HORIZON: u32 = 0u;
const OUT_OF_BOUNDING_BOX: u32 = 1u;
const CROSSED_ACCRETION_DISK: u32 = 2u;
const NO_STOPPING: u32 = 3u;

struct StoppingResult {
    criterion: u32,
    direction: vec3<f64>,
    radius: f64,
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
            return sample_skybox(stopping_result.direction);
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
    ray: ptr<function, Ray>,
    black_hole: GPUBlackHole,
    accretion_disk: GPUAccretionDisk,
    bounding_box_radius: f64,
    dl: f64
) -> StoppingResult {
    
    let current_r = (*ray).position[0];
    
    // Check event horizon
    if current_r <= black_hole.visual_radius {
        return StoppingResult(ENTERED_EVENT_HORIZON, vec3<f64>(0.0), 0.0);
    }
    
    // Solve geodesic step
    let new_position = solve_geodesic_step((*ray).position, (*ray).direction, black_hole.radius, dl);
    
    // Check accretion disk intersection
    let disk_intersection = check_accretion_intersection((*ray).position, new_position, accretion_disk);
    if disk_intersection > 0.0 {
        return StoppingResult(CROSSED_ACCRETION_DISK, vec3<f64>(0.0), disk_intersection);
    }
    
    let new_r = new_position[0];
    
    // Check bounding box + moving away
    if new_r > bounding_box_radius {
        let spatial_pos = new_position.xyz;
        let spatial_vel = (*ray).direction.xyz;
        if dot(spatial_pos, spatial_vel) > 0.0 {
            return StoppingResult(OUT_OF_BOUNDING_BOX, spatial_pos, 0.0);
        }
    }
    
    // Update ray state
    (*ray).position = new_position;
    
    return StoppingResult(NO_STOPPING, vec3<f64>(0.0), 0.0);
}
