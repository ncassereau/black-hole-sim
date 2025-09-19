#pragma once

#include "tensor_ops/tensor_ops_float3.cuh"
#include "tensor_ops/tensor_ops_double4.cuh"

template <typename T, typename U>
__device__ inline T dot(const U& a, const U& b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

template <typename T>
__device__ inline T clamp(const T& a, const T& min, const T& max) {
    if (a < min) {
        return min;
    } else if (a > max) {
        return max;
    } else {
        return a;
    }
}
