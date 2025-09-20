#pragma once

#include "color.cuh"
#include "tensor_ops.cuh"
#include <cstdio>

#define PI 3.141592654f
#define TAU 6.283185308f

struct Skybox {
    float *data;
    unsigned int width;
    unsigned int height;

    __device__ Color get_color(const float3 &direction) const {
        float3 dir = normalize(direction);
        float u = 0.5f + atan2f(dir.z, dir.x) / TAU;
        float v = acosf(dir.y) / PI;

        int x = min((int)(u * (float)width), width - 1);
        int y = min((int)(v * (float)height), height - 1);

        int index = y * width + x;
        return Color(data[index], data[index + 1], data[index + 2]);
    }
};
