#pragma once
#include "accretion_disk.cuh"
#include "black_hole.cuh"
#include "color.cuh"
#include "coordinates.cuh"
#include "hyperparameters.cuh"
#include "integration.cuh"
#include "ray.cuh"
#include "skybox.cuh"

enum class StoppingCriterion : unsigned int {
    NO_STOPPING = 0,
    ENTERED_EVENT_HORIZON = 1,
    CROSSED_ACCRETION_DISK = 2,
    OUT_OF_BOUNDING_BOX = 3,
};

struct StoppingResult {
    StoppingCriterion criterion;
    float3 direction;
    double radius;

    __device__ StoppingResult(StoppingCriterion criterion, float3 direction,
                              double radius)
        : criterion(criterion), direction(direction), radius(radius) {}

    __device__ StoppingResult(StoppingCriterion criterion, float3 direction)
        : criterion(criterion), direction(direction), radius(0.) {}

    __device__ StoppingResult(StoppingCriterion criterion, double radius)
        : criterion(criterion), direction(make_float3(0., 0., 0.)),
          radius(radius) {}

    __device__ StoppingResult(StoppingCriterion criterion)
        : criterion(criterion), direction(make_float3(0., 0., 0.)), radius(0.) {
    }

    __device__ Color determine_color(const BlackHole &black_hole,
                                     const AccretionDisk &accretion_disk,
                                     const Skybox &skybox) {
        switch (criterion) {
        case StoppingCriterion::ENTERED_EVENT_HORIZON:
            return black_hole.get_color();
        case StoppingCriterion::CROSSED_ACCRETION_DISK:
            Color color = accretion_disk.get_color(radius);
            return accretion_disk.get_color(radius);
        case StoppingCriterion::OUT_OF_BOUNDING_BOX:
            return skybox.get_color(direction);
        // We should never reach this state or default, this is only a security.
        case StoppingCriterion::NO_STOPPING:
            return Color();
        default:
            return Color();
        }
    }
};

struct StepResult {
    Ray state;
    double dl;
    StoppingResult stopping_result;

    __device__ StepResult(Ray state, double dl, StoppingResult stopping_result)
        : state(state), dl(dl), stopping_result(stopping_result) {}

    __device__ StepResult(StoppingResult stopping_result)
        : state(Ray()), dl(0.), stopping_result(stopping_result) {}

    __device__ int is_stopped() const {
        return stopping_result.criterion != StoppingCriterion::NO_STOPPING;
    }
};

__device__ StepResult step(const Ray &ray, const BlackHole &black_hole,
                           const AccretionDisk &accretion_disk,
                           const Hyperparameters &hyperparams, double dl) {
    // Check Event horizon
    if (ray.position.x <= black_hole.visual_radius)
        return StepResult(
            StoppingResult(StoppingCriterion::ENTERED_EVENT_HORIZON));

    RKResult rk_result = runge_kutta_fehlberg_45(
        ray, dl, hyperparams.integration_error_tolerance, black_hole.radius,
        hyperparams.min_dλ, hyperparams.max_dλ, hyperparams.max_dλ_ratio,
        hyperparams.max_retries);

    if (rk_result.is_stopped()) {
        if (ray.position.x <= black_hole.visual_radius)
            // RKF Step failed because we are very close to Black Hole. We
            // therefore consider that we fell into it.
            return StepResult(
                StoppingResult(StoppingCriterion::ENTERED_EVENT_HORIZON));
        else
            // Should never happen, only a safety precaution :)
            return StepResult(
                StoppingResult(StoppingCriterion::OUT_OF_BOUNDING_BOX,
                               position_to_cartesian(ray.spatial_position())));
    }

    // Check if we crossed accretion disk
    double disk_intersection = accretion_disk.check_intersection(
        position_to_cartesian(ray.spatial_position()),
        position_to_cartesian(rk_result.state.spatial_position()));
    if (disk_intersection != -1) {
        return StepResult(StoppingResult(
            StoppingCriterion::CROSSED_ACCRETION_DISK, disk_intersection));
    }

    double new_r = rk_result.state.position.x;

    // Check bounding box + moving away
    if (new_r > hyperparams.bounding_box_radius) {
        float3 pos = position_to_cartesian(ray.spatial_position());
        float3 dir = direction_to_cartesian(ray.spatial_position(),
                                            ray.spatial_direction());
        if (dot(pos, dir) > 0.) {
            // We are very far from the black hole AND we are moving away
            // from it Then early stopping. We are going to infinity so use
            // background color.
            return StepResult(
                StoppingResult(StoppingCriterion::OUT_OF_BOUNDING_BOX, pos));
        }
    }

    return StepResult(rk_result.state, rk_result.new_h,
                      StoppingResult(StoppingCriterion::NO_STOPPING));
}

__device__ Color get_ray_color(Ray ray, const BlackHole &black_hole,
                               const AccretionDisk &accretion_disk,
                               const Skybox &skybox,
                               const Hyperparameters &hyperparams) {
    Color color = Color();
    double dl = hyperparams.dλ0;

    for (unsigned int i = 0; i < hyperparams.num_integration_steps; ++i) {

        if (i > 0 && i % hyperparams.normalization_interval == 0) {
            ray.normalize_direction(black_hole.radius);
        }

        StepResult result =
            step(ray, black_hole, accretion_disk, hyperparams, dl);

        if (result.is_stopped()) {
            Color hit_color = result.stopping_result.determine_color(
                black_hole, accretion_disk, skybox);
            return hit_color;
            color.blend(hit_color);
            if (color.a > 0.95) break;
        }

        ray = result.state;
        dl = result.dl;
    }

    color.blend(Color());
    return color.gamma_correct();
}
