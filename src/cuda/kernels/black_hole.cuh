#pragma once

#include "color.cuh"

struct BlackHole {

    double radius;
    double visual_radius;
    float4 color;

    __device__ Color get_color() const { return Color(color); }
};
