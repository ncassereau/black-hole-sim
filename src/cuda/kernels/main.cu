#include "accretion_disk.cuh"
#include "black_hole.cuh"
#include "camera.cuh"
#include "hyperparameters.cuh"

extern "C" __global__
void compute(
    float *out,
    BlackHole black_hole,
    AccretionDisk accretion_disk,
    Camera camera,
    Hyperparameters hyperparams,
    size_t numel
) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < numel) {
        out[i] = 0.*black_hole.radius; //sin((float) i * (float) black_hole.radius / 1800.);
    }
}
