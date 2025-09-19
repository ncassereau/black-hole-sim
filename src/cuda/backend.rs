use cudarc::driver::{CudaContext, CudaFunction, CudaStream, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::Ptx;
use macroquad::texture::Image;
use std::error::Error;
use std::sync::Arc;

// use super::types::GPUBlackHole;
use crate::black_hole::AccretionDisk;
// use crate::gpu::GPUBackendError;
// use crate::gpu::types::{GPUAccretionDisk, GPUCamera, GPUHyperparameters};
use crate::scene::Camera;
use crate::{BLOCK_SIZE, Backend};
use crate::{
    BlackHole, CUDAAccretionDisk, CUDABlackHole, CUDACamera, CUDAHyperparameters, Hyperparameters,
    Scene,
};

pub struct CUDABackend {
    stream: Arc<CudaStream>,
    compute_kernel: CudaFunction,
}

impl CUDABackend {
    fn get_dim(numel: u32, block_size: u32) -> u32 {
        (numel + block_size - 1) / block_size
    }
}

impl Backend for CUDABackend {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let context = CudaContext::new(0)?;
        let stream = context.default_stream();
        let ptx = Ptx::from_file(env!("KERNELS_PTX_PATH"));

        let module = context.load_module(ptx)?;
        let compute_kernel = module.load_function("compute")?;

        Ok(CUDABackend {
            stream,
            compute_kernel,
        })
    }

    fn compute(
        &self,
        accretion_disk: &AccretionDisk,
        black_hole: &BlackHole,
        camera: &Camera,
        scene: &Scene,
        hyperparams: &Hyperparameters,
    ) -> Result<Image, Box<dyn Error>> {
        let hyperparams: CUDAHyperparameters = hyperparams.into();
        let black_hole: CUDABlackHole = black_hole.into();
        let accretion_disk: CUDAAccretionDisk = accretion_disk.into();
        let camera = CUDACamera::from_camera_scene(camera, scene);
        let (width, height) = scene.screen_size().unpack();
        let numel = 3 * (width * height) as usize;

        let mut cuda_output = self.stream.alloc_zeros::<f32>(numel)?;

        // self.stream.memcpy_htod(src, dst)
        let mut host_output = Vec::new();
        for _ in 0..numel {
            host_output.push(0.);
        }

        let mut builder = self.stream.launch_builder(&self.compute_kernel);
        builder.arg(&mut cuda_output);
        builder.arg(&black_hole);
        builder.arg(&accretion_disk);
        builder.arg(&camera);
        builder.arg(&hyperparams);

        unsafe {
            builder.launch(LaunchConfig {
                grid_dim: (
                    Self::get_dim(camera.screen_width, crate::BLOCK_SIZE),
                    Self::get_dim(camera.screen_height, crate::BLOCK_SIZE),
                    1,
                ),
                block_dim: (BLOCK_SIZE, BLOCK_SIZE, 1),
                shared_mem_bytes: 0,
            })?;
        };

        self.stream.synchronize()?;
        self.stream.memcpy_dtoh(&cuda_output, &mut host_output)?;

        Self::to_image(host_output, width as u16, height as u16)
    }
}
