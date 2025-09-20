#pragma diag_suppress 1444

#include <cstdio>

#include "accretion_disk.cuh"
#include "black_hole.cuh"
#include "camera.cuh"
#include "color.cuh"
#include "hyperparameters.cuh"
#include "ray.cuh"
#include "ray_tracer.cuh"
#include "skybox.cuh"

extern "C" __global__ void compute(float *output, BlackHole black_hole,
                                   AccretionDisk accretion_disk, Skybox skybox,
                                   Camera camera, Hyperparameters hyperparams) {
    unsigned int px = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int py = blockIdx.y * blockDim.y + threadIdx.y;

    if (px >= camera.screen_width || py >= camera.screen_height) return;

    unsigned int pixel_idx = py * camera.screen_width + px;

    float ndc_x = ((float)px + 0.5f) / camera.screen_width * 2.0f - 1.0f;
    float ndc_y = 1.0f - 2.0f * ((float)py + 0.5f) / camera.screen_height;

    Ray ray = camera.make_ray(ndc_x, ndc_y, black_hole.radius);
    Color color =
        get_ray_color(ray, black_hole, accretion_disk, skybox, hyperparams);

    output[pixel_idx * 3 + 0] = color.r;
    output[pixel_idx * 3 + 1] = color.g;
    output[pixel_idx * 3 + 2] = color.b;
}
