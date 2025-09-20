#pragma once

#include "tensor_ops.cuh"

__device__ float3 position_to_spherical(const float3 &pos) {
    float r = length(pos);
    float theta;
    if (r < 1e-10) {
        theta = 0.0f;
    } else {
        float cos_theta = clamp<float>(pos.z / r, -1.0f, 1.0f);
        theta = acosf(cos_theta);
    }
    float phi = atan2f(pos.y, pos.x);
    return make_float3(r, theta, phi);
}

__device__ float3 position_to_cartesian(float3 position) {
    float r = position.x;
    float theta = position.y;
    float phi = position.z;

    float sin_theta, cos_theta, sin_phi, cos_phi;
    __sincosf(theta, &sin_theta, &cos_theta);
    __sincosf(phi, &sin_phi, &cos_phi);

    float x = r * sin_theta * cos_phi;
    float y = r * sin_theta * sin_phi;
    float z = r * cos_theta;
    return make_float3(x, y, z);
}

__device__ float3 direction_to_spherical(const float3 &cartesian_pos,
                                         const float3 &spherical_pos,
                                         const float3 &cartesian_dir) {
    float dr;
    if (spherical_pos.x < 1e-10) {
        dr = 0.0f;
    } else {
        dr = dot(cartesian_pos, cartesian_dir) / spherical_pos.x;
    }

    float rho_sq =
        cartesian_pos.x * cartesian_pos.x + cartesian_pos.y * cartesian_pos.y;
    float rho = __fsqrt_rn(rho_sq);

    float dtheta;
    if (spherical_pos.x > 1e-10 && rho > 1e-10) {
        dtheta = (cartesian_pos.z * (cartesian_pos.x * cartesian_dir.x +
                                     cartesian_pos.y * cartesian_dir.y) -
                  rho_sq * cartesian_dir.z) /
                 (spherical_pos.x * spherical_pos.x * rho);
    } else {
        dtheta = 0.0f;
    }

    float dphi;
    if (rho_sq > 1e-10) {
        dphi = (cartesian_pos.x * cartesian_dir.y -
                cartesian_dir.x * cartesian_pos.y) /
               rho_sq;
    } else {
        dphi = 0.0f;
    }

    return make_float3(dr, dtheta, dphi);
}

__device__ float3 direction_to_cartesian(float3 position, float3 direction) {
    float r = position.x;
    float theta = position.y;
    float phi = position.z;
    float dr = direction.x;
    float dtheta = direction.y;
    float dphi = direction.z;

    float sin_theta, cos_theta, sin_phi, cos_phi;
    __sincosf(theta, &sin_theta, &cos_theta);
    __sincosf(phi, &sin_phi, &cos_phi);

    float sin_theta_cos_phi = sin_theta * cos_phi;
    float sin_theta_sin_phi = sin_theta * sin_phi;

    float dx = dr * sin_theta_cos_phi + r * dtheta * cos_theta * cos_phi -
               r * dphi * sin_theta_sin_phi;
    float dy = dr * sin_theta_sin_phi + r * dtheta * cos_theta * sin_phi +
               r * dphi * sin_theta_cos_phi;
    float dz = dr * cos_theta - r * dtheta * sin_theta;
    return make_float3(dx, dy, dz);
}
