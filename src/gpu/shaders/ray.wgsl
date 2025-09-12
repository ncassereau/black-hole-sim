struct Ray {
    position: vec4<f64>,
    direction: vec4<f64>,
}


fn from_camera_to_world_coordinates(
    ray_direction: vec3<f64>, camera: GPUCamera
) -> vec3<f64> {
    return normalize(
        camera.right.xyz * ray_direction[0]
        + camera.up.xyz * ray_direction[1]
        + camera.forward.xyz * ray_direction[2]
    );
}

fn get_ray(ndc_x: f64, ndc_y: f64, camera: GPUCamera, rs: f64) -> Ray {
    var ray_direction = vec3(
        ndc_x * camera.scale * camera.aspect_ratio,
        ndc_y * camera.scale,
        1.
    );
    ray_direction = from_camera_to_world_coordinates(ray_direction, camera);

    let cart_position = vec3<f32>(camera.position.xyz);
    let sph_position = position_to_spherical(cart_position);
    let sph_direction = direction_to_spherical(
        cart_position,
        sph_position,
        vec3<f32>(ray_direction),
    );

    let position_4d = vec4(vec3<f64>(sph_position), 0.);

    let direction_4d = vec4(vec3<f64>(sph_direction), 0.);
    let norm_direction_4d = normalize_direction(position_4d, direction_4d, rs);
    return Ray(position_4d, direction_4d);
}

fn normalize_direction(position: vec4<f64>, direction: vec4<f64>, rs: f64) -> vec4<f64> {
    let r2 = position[0] * position[0];
    var denom = (1.0 - rs / position[0]);
    if denom < 1e-10 {
        denom = 1e-10;
    }

    let dir2 = direction * direction;
    let num_part1 = (1.0 / denom) * dir2[0];
    let num_part2 = r2 * dir2[1];
    let _sin_theta_dphi = sin(position[1]) * direction[2];
    let num_part3 = r2 * _sin_theta_dphi * _sin_theta_dphi;

    let dt = sqrt((num_part1 + num_part2 + num_part3) / denom);
    return vec4(direction.xyz, dt);
}

fn position_to_spherical(pos: vec3<f32>) -> vec3<f32> {
    let r = length(pos);
    var theta: f32;
    if r < 1e-10 {
        theta = 0.0;
    } else {
        let cos_theta = clamp(pos[2] / r, -1.0, 1.0);
        theta = acos(cos_theta);
    }
    let phi = atan2(pos[1], pos[0]);
    return vec3(r, theta, phi);
}

fn direction_to_spherical(
    cart_position: vec3<f32>,
    sph_position: vec3<f32>,
    direction: vec3<f32>,
) -> vec3<f32> {
    var dr: f32;
    if sph_position[0] < 1e-10 {
        dr = 0.;
    } else {
        dr = dot(cart_position, direction) / sph_position[0];
    }

    let rho_sq = cart_position[0] * cart_position[0] + cart_position[1] * cart_position[1];
    let rho = sqrt(rho_sq);

    var dtheta: f32;
    if sph_position[0] > 1e-10 && rho > 1e-10 {
        dtheta = (cart_position[2] * (cart_position[0] * direction[0] + cart_position[1] * direction[1]) - rho_sq * direction[2]) / (sph_position[0] * sph_position[0] * rho);
    } else {
        dtheta = 0.;
    }

    var dphi: f32;
    if rho_sq > 1e-10 {
        dphi = (cart_position[0] * direction[1] - direction[0] * cart_position[1]) / rho_sq;
    } else {
        dphi = 0.;
    }
    return vec3(dr, dtheta, dphi);
}

fn get_color(
    ray: ptr<function, Ray>,
    black_hole: GPUBlackHole,
    hyperparams: GPUHyperparameters,
) {
    var accumulated_color: vec4<f32> = vec4(0., 0., 0., 0.);
    var transmittance: f32 = 1.0;

    for (var i: u32 = 0u; i < hyperparams.num_integration_steps; i++) {
        if (i > 0u && i % hyperparams.normalization_interval == 0u) {
            (*ray).direction = normalize_direction((*ray).position, (*ray).direction, black_hole.radius);
        }

        let hit_something = step_ray(ray, black_hole, bounding_box_radius);
        
        if (hit_something) {
            let hit_color = determine_color((*ray).position, black_hole);
            
            let blended = blend_colors(accumulated_color, hit_color, transmittance);
            accumulated_color = blended.rgb;
            transmittance = blended.a;
            
            if (transmittance < 0.05) {
                break;
            }
        }
    }
}