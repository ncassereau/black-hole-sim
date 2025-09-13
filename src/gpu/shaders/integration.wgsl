struct RKResult {
    state: GPURay,
    error: f64,
    new_h: f64,
    return_code: u32,
}

fn rkf_k1(ray: GPURay, h: f64, rs: f64) -> GPURay {
    return GPURay(ray.direction, geodesic(ray, rs));
}

fn _rkf_k2(v: vec4<f64>, h:f64, k1_val: vec4<f64>) -> vec4<f64> {
    return v + k1_val * (h * (1.0 / 4.0));
}

fn rkf_k2(ray: GPURay, h: f64, rs: f64, k1_val: GPURay) -> GPURay {
    let state = GPURay(
        _rkf_k2(ray.position, h, k1_val.position),
        _rkf_k2(ray.direction, h, k1_val.direction),
    );
    return GPURay(state.direction, geodesic(state, rs));
}

fn _rkf_k3(v: vec4<f64>, h: f64, k1_val: vec4<f64>, k2_val: vec4<f64>) -> vec4<f64> {
    return v + k1_val * (h * (3.0 / 32.0)) + k2_val * (h * (9.0 / 32.0));
}

fn rkf_k3(ray: GPURay, h: f64, rs: f64, k1_val: GPURay, k2_val: GPURay) -> GPURay {
    let state = GPURay(
        _rkf_k3(ray.position, h, k1_val.position, k2_val.position),
        _rkf_k3(ray.direction, h, k1_val.direction, k2_val.direction),
    );
    return GPURay(state.direction, geodesic(state, rs));
}

fn _rkf_k4(v: vec4<f64>, h: f64, k1_val: vec4<f64>, k2_val: vec4<f64>, k3_val: vec4<f64>) -> vec4<f64> {
    return v + k1_val * (h * (1932.0 / 2197.0)) - k2_val * (h * (7200.0 / 2197.0)) + k3_val * (h * (7296.0 / 2197.0));
}

fn rkf_k4(ray: GPURay, h: f64, rs: f64, k1_val: GPURay, k2_val: GPURay, k3_val: GPURay) -> GPURay {
    let state = GPURay(
        _rkf_k4(ray.position, h, k1_val.position, k2_val.position, k3_val.position),
        _rkf_k4(ray.direction, h, k1_val.direction, k2_val.direction, k3_val.direction),
    );
    return GPURay(state.direction, geodesic(state, rs));
}

fn _rkf_k5(v: vec4<f64>, h: f64, k1_val: vec4<f64>, k2_val: vec4<f64>, k3_val: vec4<f64>, k4_val: vec4<f64>) -> vec4<f64> {
    return v + k1_val * (h * (439.0 / 216.0)) - k2_val * (h * 8.0) + k3_val * (h * (3680.0 / 513.0)) - k4_val * (h * (845.0 / 4104.0));
}

fn rkf_k5(ray: GPURay, h: f64, rs: f64, k1_val: GPURay, k2_val: GPURay, k3_val: GPURay, k4_val: GPURay) -> GPURay {
    let state = GPURay(
        _rkf_k5(ray.position, h, k1_val.position, k2_val.position, k3_val.position, k4_val.position),
        _rkf_k5(ray.direction, h, k1_val.direction, k2_val.direction, k3_val.direction, k4_val.direction),
    );
    return GPURay(state.direction, geodesic(state, rs));
}

fn _rkf_k6(v: vec4<f64>, h: f64, k1_val: vec4<f64>, k2_val: vec4<f64>, k3_val: vec4<f64>, k4_val: vec4<f64>, k5_val: vec4<f64>) -> vec4<f64> {
    return v - k1_val * (h * (8.0 / 27.0)) + k2_val * (h * 2.0) - k3_val * (h * (3544.0 / 2565.0)) + k4_val * (h * (1859.0 / 4104.0)) - k5_val * (h * (11.0 / 40.0));
}

fn rkf_k6(ray: GPURay, h: f64, rs: f64, k1_val: GPURay, k2_val: GPURay, k3_val: GPURay, k4_val: GPURay, k5_val: GPURay) -> GPURay {
    let state = GPURay(
        _rkf_k6(ray.position, h, k1_val.position, k2_val.position, k3_val.position, k4_val.position, k5_val.position),
        _rkf_k6(ray.direction, h, k1_val.direction, k2_val.direction, k3_val.direction, k4_val.direction, k5_val.direction),
    );
    return GPURay(state.direction, geodesic(state, rs));
}

fn _order_4(v: vec4<f64>, h: f64, k1_val: vec4<f64>, k3_val: vec4<f64>, k4_val: vec4<f64>, k5_val: vec4<f64>) -> vec4<f64> {
    return v + k1_val * (h * (25.0 / 216.0)) + k3_val * (h * (1408.0 / 2565.0)) + k4_val * (h * (2197.0 / 4104.0)) - k5_val * (h * (1.0 / 5.0));
}

fn _order_5(v: vec4<f64>, h: f64, k1_val: vec4<f64>, k3_val: vec4<f64>, k4_val: vec4<f64>, k5_val: vec4<f64>, k6_val: vec4<f64>) -> vec4<f64> {
    return v + k1_val * (h * (16.0 / 135.0)) + k3_val * (h * (6656.0 / 12825.0)) + k4_val * (h * (28561.0 / 56430.0)) - k5_val * (h * (9.0 / 50.0)) + k6_val * (h * (2.0 / 55.0));
}

fn runge_kutta_fehlberg_45_pseudo_step(
    ray: GPURay,
    h: f64,
    tolerance: f64,
    rs: f64,
) -> RKResult {
    let k1_val = rkf_k1(ray, h, rs);
    let k2_val = rkf_k2(ray, h, rs, k1_val);
    let k3_val = rkf_k3(ray, h, rs, k1_val, k2_val);
    let k4_val = rkf_k4(ray, h, rs, k1_val, k2_val, k3_val);
    let k5_val = rkf_k5(ray, h, rs, k1_val, k2_val, k3_val, k4_val);
    let k6_val = rkf_k6(ray, h, rs, k1_val, k2_val, k3_val, k4_val, k5_val);
    
    let state_4 = GPURay(
        _order_4(ray.position, h, k1_val.position, k3_val.position, k4_val.position, k5_val.position),
        _order_4(ray.direction, h, k1_val.direction, k3_val.direction, k4_val.direction, k5_val.direction)
    );
    
    let state_5 = GPURay(
        _order_5(ray.position, h, k1_val.position, k3_val.position, k4_val.position, k5_val.position, k6_val.position),
        _order_5(ray.direction, h, k1_val.direction, k3_val.direction, k4_val.direction, k5_val.direction, k6_val.direction)
    );
    
    let pos_error = state_4.position - state_5.position;
    let dir_error = state_4.direction - state_5.direction;
    let error = sqrt(dot(pos_error, pos_error) + dot(dir_error, dir_error));
    
    let new_h = 0.9 * h * pow(tolerance / max(error, 1e-10), 0.2);

    return RKResult(state_5, error, new_h, NO_STOPPING);
}

fn runge_kutta_fehlberg_45(
    ray: GPURay,
    h: f64,
    tolerance: f64,
    rs: f64,
    min_h: f64,
    max_h: f64,
    max_retries: u64,
) -> RKResult {
    var current_h = h;
    var counter: u64 = 0;

    for (var counter: u64 = 0; counter <= max_retries; counter++) {
        var result = runge_kutta_fehlberg_45_pseudo_step(
            ray, current_h, tolerance, rs
        );

        if result.error < tolerance {
            return result;
        }

        if result.new_h < min_h {
            result.return_code = MIN_STEP_REACHED;
            return result;
        }

        current_h = result.new_h;
    }
    
    return RKResult(ray, 0.0, current_h, MAX_RETRIES_REACHED);
}