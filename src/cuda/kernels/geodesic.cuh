#pragma once

#include "ray.cuh"

__device__ double4 geodesic(const Ray &ray, double rs) {
    double r = ray.position.x;
    double theta = ray.position.y;
    double phi = ray.position.z;
    double t = ray.position.w;

    double dr = ray.direction.x;
    double dtheta = ray.direction.y;
    double dphi = ray.direction.z;
    double dt = ray.direction.w;

    double sin_theta;
    double cos_theta;
    sincos(theta, &sin_theta, &cos_theta);
    double altitude = fmax(r - rs, 1e-10);

    double d2t = -rs * dr * dt / (r * altitude);

    double r_term1 = rs * altitude * dt * dt / (2.0 * r * r * r);
    double r_term2 = rs / (2.0 * r * altitude) * dr * dr;
    double r_term3 = dtheta * dtheta + (sin_theta * dphi) * (sin_theta * dphi);
    double d2r = -r_term1 + r_term2 + altitude * r_term3;

    double theta_term1 = -2.0 * dr * dtheta / r;
    double theta_term2 = sin_theta * cos_theta * dphi * dphi;
    double d2theta = theta_term1 + theta_term2;

    double phi_term1 = dr / r;
    double phi_term2 =
        (fabs(sin_theta) > 1e-10) ? dtheta * cos_theta / sin_theta : 0.0;
    double d2phi = -2.0 * dphi * (phi_term1 + phi_term2);

    return make_double4(d2r, d2theta, d2phi, d2t);
}
