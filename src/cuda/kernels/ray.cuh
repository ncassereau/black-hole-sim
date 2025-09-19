#include "tensor_ops.cuh"

struct Ray {
    double4 position;
    double4 direction;

    __device__ Ray(const double4& pos, const double4& dir) : position(pos), direction(dir) {}

    __device__ Ray(const float3& pos, const float3& dir, double rs) {
        position = make_double4(
            (double) pos.x,
            (double) pos.y,
            (double) pos.z,
            0.0
        );
        direction = make_double4(
            (double) dir.x,
            (double) dir.y,
            (double) dir.z,
            0.0
        );
        normalize_direction(rs);
    }

    __device__ void normalize_direction(double rs) {
        double r2 = position.x * position.x;
        double denom = (1.0 - rs / position.x);
        if (denom < 1e-10) {
            denom = 1e-10;
        }

        double4 dir2 = direction * direction;
        double num_part1 = dir2.x / denom;
        double num_part2 = r2 * dir2.y;
        double _sin_theta_dphi = __sinf(position.y) * direction.z;
        double num_part3 = r2 * _sin_theta_dphi * _sin_theta_dphi;

        direction.w = __dsqrt_rn(
            (num_part1 + num_part2 + num_part3) / denom
        );
    }

};
