struct GPUAccretionDisk {
    r_isco: f64,
    accretion_r_max: f64,
    width: f64,
    max_temperature: f64,
    step_opacity: f64,
    doppler_factor: f64,
    fade_start_ratio: f64,
    peak_brigthness: f64,
}


fn check_accretion_intersection(
    accredition_disk: GPUAccretionDisk,
    position1: vec3<f64>,
    position2: vec3<f64>,
) -> f64 {
    let t = -position1[2] / (position2[2] - poistion1[2]);
    if t < 0. || t > 1. {
        return -1;
    }
    let equator_collision = position1 + (position2 - position1) * t;
    let r_plane = length(equator_collision);
    if r_plane < accredition_disk.r_isco || r_plane > accredition_disk.accretion_r_max {
        return -1;
    }
    return r_plane;
}

fn kelvin_to_rgb(temp_kelvin: f32) -> vec3<f32> {
    let temp = temp_kelvin / 100.0;
    
    var red: f32;
    if (temp <= 66.0) {
        red = 255.0;
    } else {
        red = 329.698727446 * pow(temp - 60.0, -0.1332047592);
    }
    
    var green: f32;
    if (temp <= 66.0) {
        green = 99.4708025861 * log(temp) - 161.1195681661;
    } else {
        green = 288.1221695283 * pow(temp - 60.0, -0.0755148492);
    }
    
    var blue: f32;
    if (temp <= 19.0) {
        blue = 0.0;
    } else if (temp >= 66.0) {
        blue = 255.0;
    } else {
        blue = 138.5177312231 * log(temp - 10.0) - 305.0447927307;
    }
    
    return vec3<f32>(
        clamp(red, 0.0, 255.0) / 255.0,
        clamp(green, 0.0, 255.0) / 255.0,
        clamp(blue, 0.0, 255.0) / 255.0
    );
}

fn brightness(r_isco: f32, radius: f32) -> f32 {
    return (1.0 - sqrt(r_isco / radius)) / (pow(radius, 3.0) + 1e-10);
}

fn get_accretion_disk_color(accretion_disk: GPUAccretionDisk, radius: f64) -> GPUColor {
    if (radius < disk.r_isco || radius > disk.accretion_r_max) {
        return Color(0.0, 0.0, 0.0, 0.0);
    }
    
    // Shakura-Sunyaev
    let normalized_brightness = min(brightness(disk.r_isco, radius) / disk.peak_brightness, 1.0);
    
    let normalized_radius = (radius - disk.r_isco) / disk.width;
    let geometric_falloff = 1.0 - smoothstep(disk.fade_start_ratio, 1.0, normalized_radius);
    
    // Stefan-Boltzmann's Law
    let temp_k = disk.max_temperature * pow(normalized_brightness, 0.25);
    
    // Planck's Law
    let rgb = kelvin_to_rgb(temp_k);
    
    // Effet Doppler (désactivé pour l'instant)
    let doppler_boost = 1.0;
    
    let emitted_intensity = normalized_brightness * doppler_boost * geometric_falloff;
    
    let emitted_r = rgb.r * emitted_intensity;
    let emitted_g = rgb.g * emitted_intensity;
    let emitted_b = rgb.b * emitted_intensity;
    let opacity_for_step = disk.step_opacity * emitted_intensity;
    
    return Color(emitted_r, emitted_g, emitted_b, opacity_for_step);
}
