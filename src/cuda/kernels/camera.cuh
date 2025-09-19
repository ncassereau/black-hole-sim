#include "cuda_runtime.h"

#include "tensor_ops.cuh"
#include "ray.cuh"

__device__ float3 position_to_spherical(const float3& pos) {
    float r =  length(pos);
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

__device__ float3 direction_to_spherical(
    const float3& cartesian_pos,
    const float3& spherical_pos,
    const float3& cartesian_dir
) {
    float dr;
    if (spherical_pos.x < 1e-10) {
        dr = 0.0f;
    } else {
        dr = dot(cartesian_pos, cartesian_dir) / spherical_pos.x;
    }

    float rho_sq = cartesian_pos.x * cartesian_pos.x + cartesian_pos.y * cartesian_pos.y;
    float rho = __fsqrt_rn(rho_sq);

    float dtheta;
    if (spherical_pos.x > 1e-10 && rho > 1e-10) {
        dtheta = (cartesian_pos.z * (cartesian_pos.x * cartesian_dir.x + cartesian_pos.y * cartesian_dir.y) - rho_sq * cartesian_dir.z) / (spherical_pos.x * spherical_pos.x * rho);
    } else {
        dtheta = 0.0f;
    }

    float dphi;
    if (rho_sq > 1e-10) {
        dphi = (cartesian_pos.x * cartesian_dir.y - cartesian_dir.x * cartesian_pos.y) / rho_sq;
    } else {
        dphi = 0.0f;
    }

    return make_float3(dr, dtheta, dphi);
}


struct Camera {
    float3 position;
    float3 right;
    float3 up;
    float3 forward;
    float scale; // This is the tangent of FOV
    float aspect_ratio; // Width / height
    unsigned int screen_width;
    unsigned int screen_height;

    __device__ Camera(
        float3 pos,
        float3 r,
        float3 u,
        float3 f,
        double s,
        double ar,
        unsigned int sw,
        unsigned int sh
    ) : position(pos), right(r), up(u), forward(f),
        scale(s), aspect_ratio(ar), screen_width(sw), screen_height(sh) {}

    __device__ float3 convert_vector_to_world_coordinates(
        const float3& v
    ) const {
        return normalize(
            right * v.x
            + up * v.y
            + forward * v.z
        );
    }

    __device__ Ray make_ray(float ndc_x, float ndc_y, double rs) const {
        float3 ray_dir = make_float3(
            ndc_x * scale * aspect_ratio,
            ndc_y * scale,
            1.
        );
        ray_dir = convert_vector_to_world_coordinates(ray_dir);
        float3 spherical_pos = position_to_spherical(position);
        float3 spherical_dir = direction_to_spherical(
            position, spherical_pos, ray_dir
        );

        return Ray(spherical_pos, spherical_dir, rs);
    }
};
