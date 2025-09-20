#pragma once

#include "geodesic.cuh"
#include "ray.cuh"
#include "tensor_ops.cuh"

enum class RKReturnCode : unsigned int {
    NO_STOPPING = 0,
    MIN_STEP_REACHED = 1,
    MAX_RETRIES_REACHED = 2
};

struct RKResult {
    Ray state;
    double error;
    double new_h;
    RKReturnCode return_code;

    __device__ RKResult(Ray s, double e, double h, RKReturnCode rc)
        : state(s), error(e), new_h(h), return_code(rc) {}

    __device__ inline int is_stopped() {
        return return_code != RKReturnCode::NO_STOPPING;
    }
};

__device__ Ray rkf_k1(const Ray &ray, double h, double rs) {
    return Ray(ray.direction, geodesic(ray, rs));
}

__device__ Ray rkf_k2(const Ray &ray, double h, double rs, const Ray &k1_val) {
    Ray state = ray + k1_val * (h * (1.0 / 4.0));
    return Ray(state.direction, geodesic(state, rs));
}

__device__ Ray rkf_k3(const Ray &ray, double h, double rs, const Ray &k1_val,
                      const Ray &k2_val) {
    Ray state = ray + k1_val * (h * (3.0 / 32.0)) + k2_val * (h * (9.0 / 32.0));
    return Ray(state.direction, geodesic(state, rs));
}

__device__ Ray rkf_k4(const Ray &ray, double h, double rs, const Ray &k1_val,
                      const Ray &k2_val, const Ray &k3_val) {
    Ray state = ray + k1_val * (h * (1932.0 / 2197.0)) -
                k2_val * (h * (7200.0 / 2197.0)) +
                k3_val * (h * (7296.0 / 2197.0));
    return Ray(state.direction, geodesic(state, rs));
}

__device__ Ray rkf_k5(const Ray &ray, double h, double rs, const Ray &k1_val,
                      const Ray &k2_val, const Ray &k3_val, const Ray &k4_val) {
    Ray state = ray + k1_val * (h * (439.0 / 216.0)) - k2_val * (h * 8.0) +
                k3_val * (h * (3680.0 / 513.0)) -
                k4_val * (h * (845.0 / 4104.0));
    return Ray(state.direction, geodesic(state, rs));
}

__device__ Ray rkf_k6(const Ray &ray, double h, double rs, const Ray &k1_val,
                      const Ray &k2_val, const Ray &k3_val, const Ray &k4_val,
                      const Ray &k5_val) {
    Ray state = ray - k1_val * (h * (8.0 / 27.0)) + k2_val * (h * 2.0) -
                k3_val * (h * (3544.0 / 2565.0)) +
                k4_val * (h * (1859.0 / 4104.0)) - k5_val * (h * (11.0 / 40.0));
    return Ray(state.direction, geodesic(state, rs));
}

__device__ Ray order_4(const Ray &ray, double h, const Ray &k1_val,
                       const Ray &k3_val, const Ray &k4_val,
                       const Ray &k5_val) {
    return ray + k1_val * (h * (25.0 / 216.0)) +
           k3_val * (h * (1408.0 / 2565.0)) + k4_val * (h * (2197.0 / 4104.0)) -
           k5_val * (h * (1.0 / 5.0));
}

__device__ Ray order_5(const Ray &ray, double h, const Ray &k1_val,
                       const Ray &k3_val, const Ray &k4_val, const Ray &k5_val,
                       const Ray &k6_val) {
    return ray + k1_val * (h * (16.0 / 135.0)) +
           k3_val * (h * (6656.0 / 12825.0)) +
           k4_val * (h * (28561.0 / 56430.0)) - k5_val * (h * (9.0 / 50.0)) +
           k6_val * (h * (2.0 / 55.0));
}

__device__ RKResult runge_kutta_fehlberg_45_pseudo_step(const Ray &ray,
                                                        double h,
                                                        double tolerance,
                                                        double rs) {
    Ray k1_val = rkf_k1(ray, h, rs);
    Ray k2_val = rkf_k2(ray, h, rs, k1_val);
    Ray k3_val = rkf_k3(ray, h, rs, k1_val, k2_val);
    Ray k4_val = rkf_k4(ray, h, rs, k1_val, k2_val, k3_val);
    Ray k5_val = rkf_k5(ray, h, rs, k1_val, k2_val, k3_val, k4_val);
    Ray k6_val = rkf_k6(ray, h, rs, k1_val, k2_val, k3_val, k4_val, k5_val);

    Ray state_4 = order_4(ray, h, k1_val, k3_val, k4_val, k5_val);
    Ray state_5 = order_5(ray, h, k1_val, k3_val, k4_val, k5_val, k6_val);

    Ray error_ray = state_4 - state_5;
    double error = __dsqrt_rn(
        dot<double, double4>(error_ray.position, error_ray.position) +
        dot<double, double4>(error_ray.direction, error_ray.direction));

    double new_h = 0.9 * h * pow(tolerance / fmax(error, 1e-10), 0.2);

    return RKResult(state_5, error, new_h, RKReturnCode::NO_STOPPING);
}

__device__ RKResult runge_kutta_fehlberg_45(const Ray &ray, double h,
                                            double tolerance, double rs,
                                            double min_h, double max_h,
                                            double max_h_ratio,
                                            unsigned int max_retries) {
    double current_h = h;

    for (unsigned int counter = 0; counter <= max_retries; counter++) {
        RKResult result =
            runge_kutta_fehlberg_45_pseudo_step(ray, current_h, tolerance, rs);

        if (result.error < tolerance) {
            result.new_h =
                min(min(result.new_h, current_h * max_h_ratio), max_h);
            return result;
        }

        if (result.new_h < min_h) {
            return RKResult(result.state, result.error, result.new_h,
                            RKReturnCode::MIN_STEP_REACHED);
        }

        current_h = result.new_h;
    }

    return RKResult(ray, 0.0, current_h, RKReturnCode::MAX_RETRIES_REACHED);
}
