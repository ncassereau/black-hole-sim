#pragma once

#include "tensor_ops/tensor_ops_double3.cuh"
#include "tensor_ops/tensor_ops_double4.cuh"
#include "tensor_ops/tensor_ops_float3.cuh"

template <typename T, typename U>
__device__ inline T dot(const U &a, const U &b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

template <typename T>
__device__ inline T clamp(const T &a, const T &min, const T &max) {
    if (a < min) {
        return min;
    } else if (a > max) {
        return max;
    } else {
        return a;
    }
}

__device__ inline double3 to_double3(const double4 &vec) {
    return make_double3(vec.x, vec.y, vec.z);
}

__device__ double smoothstep(double edge0, double edge1, double x) {
    x = clamp<double>((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return x * x * (3.0f - 2.0f * x);
}
