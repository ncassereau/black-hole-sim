struct GPUAccretionDisk {
    r_isco: f64,
    accretion_r_max: f64,
    width: f64,
    max_temperature: f64,
    step_opacity: f64,
    doppler_factor: f64,
    fade_start_ratio: f64,
    peak_brightness: f64,
}


fn check_accretion_intersection(
    accretion_disk: GPUAccretionDisk,
    position1: vec3<f64>,
    position2: vec3<f64>,
) -> f64 {
    let dz = position2[2] - position1[2];
    if abs(dz) < 1e-10 {
        return -1;
    }
    let t = -position1[2] / dz;
    if t < 0. || t > 1. {
        return -1;
    }
    let equator_collision = position1 + (position2 - position1) * t;
    let r_plane = length(equator_collision);
    if r_plane < accretion_disk.r_isco || r_plane > accretion_disk.accretion_r_max {
        return -1;
    }
    return r_plane;
}

fn kelvin_to_rgb(temp_kelvin: f32) -> vec3<f32> {
    let temp = temp_kelvin / 100.0;
    
    let red = select(
        255.0,
        329.698727446 * pow(temp - 60.0, -0.1332047592),
        temp <= 66.0,
    );
    
    let green = select(
        99.4708025861 * log(temp) - 161.1195681661,
        288.1221695283 * pow(temp - 60.0, -0.0755148492),
        temp <= 66.0,
    );
    
    let blue = select(
        0.,
        select(
            255.0,
            138.5177312231 * log(temp - 10.0) - 305.0447927307,
            temp >= 66.0,
        ),
        temp <= 19.0,
    );
    
    return vec3<f32>(
        clamp(red, 0.0, 255.0) / 255.0,
        clamp(green, 0.0, 255.0) / 255.0,
        clamp(blue, 0.0, 255.0) / 255.0
    );
}

fn brightness(r_isco: f64, radius: f64) -> f64 {
    return (1.0 - sqrt(r_isco / radius)) / (pow(radius, 3.0) + 1e-10);
}

fn get_accretion_disk_color(disk: GPUAccretionDisk, radius: f64) -> GPUColor {
    if (radius < disk.r_isco || radius > disk.accretion_r_max) {
        return GPUColor(0.0, 0.0, 0.0, 0.0);
    }
    
    // Shakura-Sunyaev
    let normalized_brightness = min(brightness(disk.r_isco, radius) / disk.peak_brightness, 1.0);
    
    let normalized_radius = (radius - disk.r_isco) / disk.width;
    let geometric_falloff = 1.0 - smoothstep(disk.fade_start_ratio, 1.0, normalized_radius);
    
    // Stefan-Boltzmann's Law
    let temp_k = disk.max_temperature * pow(normalized_brightness, 0.25);
    
    // Planck's Law
    let rgb = kelvin_to_rgb(f32(temp_k));
    
    // Effet Doppler (disabled for now)
    let doppler_boost: f64 = 1.0;

    let emitted_intensity = f32(normalized_brightness * doppler_boost * geometric_falloff);
    
    let emitted_r = rgb.r * emitted_intensity;
    let emitted_g = rgb.g * emitted_intensity;
    let emitted_b = rgb.b * emitted_intensity;
    let opacity_for_step = f32(disk.step_opacity) * emitted_intensity;
    
    return GPUColor(emitted_r, emitted_g, emitted_b, opacity_for_step);
}
