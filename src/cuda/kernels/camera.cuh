#pragma once

#include "cuda_runtime.h"

#include "coordinates.cuh"
#include "ray.cuh"
#include "tensor_ops.cuh"

struct Camera {
    float3 position;
    float3 right;
    float3 up;
    float3 forward;
    float scale;        // This is the tangent of FOV
    float aspect_ratio; // Width / height
    unsigned int screen_width;
    unsigned int screen_height;

    __device__ Camera(float3 pos, float3 r, float3 u, float3 f, double s,
                      double ar, unsigned int sw, unsigned int sh)
        : position(pos), right(r), up(u), forward(f), scale(s),
          aspect_ratio(ar), screen_width(sw), screen_height(sh) {}

    __device__ float3
    convert_vector_to_world_coordinates(const float3 &v) const {
        return normalize(right * v.x + up * v.y + forward * v.z);
    }

    __device__ Ray make_ray(float ndc_x, float ndc_y, double rs) const {
        float3 ray_dir =
            make_float3(ndc_x * scale * aspect_ratio, ndc_y * scale, 1.);
        ray_dir = convert_vector_to_world_coordinates(ray_dir);
        float3 spherical_pos = position_to_spherical(position);
        float3 spherical_dir =
            direction_to_spherical(position, spherical_pos, ray_dir);

        return Ray(spherical_pos, spherical_dir, rs);
    }
};
