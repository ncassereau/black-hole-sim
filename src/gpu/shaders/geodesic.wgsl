fn geodesic(ray: GPURay, rs: f64) -> vec4<f64> {
    let r = ray.position[0];
    let theta = ray.position[1];
    let phi = ray.position[2];
    let t = ray.position[3];

    let dr = ray.direction[0];
    let dtheta = ray.direction[1];
    let dphi = ray.direction[2];
    let dt = ray.direction[3];

    let sin_theta = sin(theta);
    let cos_theta = cos(theta);
    let altitude = max(r - rs, 1e-10);

    let d2t = -rs * dr * dt / (r * altitude);

    let r_term1 = rs * altitude * dt * dt / (2.0 * r * r * r);
    let r_term2 = rs / (2.0 * r * altitude) * dr * dr;
    let r_term3 = dtheta * dtheta + (sin_theta * dphi) * (sin_theta * dphi);
    let d2r = -r_term1 + r_term2 + altitude * r_term3;

    let theta_term1 = -2.0 * dr * dtheta / r;
    let theta_term2 = sin_theta * cos_theta * dphi * dphi;
    let d2theta = theta_term1 + theta_term2;

    let phi_term1 = dr / r;
    let phi_term2 = select(0.0, dtheta * cos_theta / sin_theta, abs(sin_theta) > 1e-10);
    let d2phi = -2.0 * dphi * (phi_term1 + phi_term2);

    return vec4<f64>(d2r, d2theta, d2phi, d2t);
}