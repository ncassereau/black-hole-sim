struct GPUColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

struct GPUColorState {
    color: GPUColor,
    transmittance: f32,
}

const INVERSE_GAMMA: f32 = 1.0 / 2.2;

fn blend_color(
    accumulated_color: GPUColorState,
    sample_color: GPUColor,
) -> GPUColorState {
    let transmittance = accumulated_color.transmittance;
    let acc_color = accumulated_color.color; 
    let new_transmittance = transmittance * (1.0 - sample_color.a);
    
    let r = acc_color.r + sample_color.r * sample_color.a * transmittance;
    let g = acc_color.g + sample_color.g * sample_color.a * transmittance;
    let b = acc_color.b + sample_color.b * sample_color.a * transmittance;
    
    let blended_color = GPUColor(r, g, b, 1.0 - new_transmittance);
    
    return GPUColorState(blended_color, new_transmittance);
}

fn gamma_correct(linear_color: GPUColor) -> GPUColor {
    return GPUColor(
        pow(linear_color.r, INVERSE_GAMMA),
        pow(linear_color.g, INVERSE_GAMMA),
        pow(linear_color.b, INVERSE_GAMMA),
        linear_color.a
    );
}