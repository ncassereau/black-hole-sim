
use cudarc::driver::{CudaContext, CudaFunction, LaunchConfig, CudaStream, PushKernelArg};
use cudarc::nvrtc::Ptx;
use macroquad::texture::Image;
use std::sync::Arc;
use std::error::Error;

// use super::types::GPUBlackHole;
use crate::black_hole::AccretionDisk;
// use crate::gpu::GPUBackendError;
// use crate::gpu::types::{GPUAccretionDisk, GPUCamera, GPUHyperparameters};
use crate::scene::Camera;
use crate::{BlackHole, CUDAAccretionDisk, CUDABlackHole, CUDACamera, CUDAHyperparameters, Hyperparameters, Scene};

pub struct CUDABackend {
    context: Arc<CudaContext>,
    stream: Arc<CudaStream>,
    compute_kernel: CudaFunction,
}

impl CUDABackend {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let context = CudaContext::new(0)?;
        let stream = context.default_stream();
        let ptx = Ptx::from_file(env!("KERNELS_PTX_PATH"));
        
        let module = context.load_module(ptx)?;
        let compute_kernel = module.load_function("compute")?;

        Ok(CUDABackend {
            context,
            stream,
            compute_kernel,
        })
    }

    pub fn compute(
        &self,
        accretion_disk: &AccretionDisk,
        black_hole: &BlackHole,
        camera: &Camera,
        scene: &Scene,
        hyperparams: &Hyperparameters,
    ) -> Image {
        let hyperparams: CUDAHyperparameters = hyperparams.into();
        let black_hole: CUDABlackHole = black_hole.into();
        let accretion_disk: CUDAAccretionDisk = accretion_disk.into();
        let camera = CUDACamera::from_camera_scene(camera, scene);
        let (width, height) = scene.screen_size().unpack();
        let numel = (width * height) as usize;

        let mut cudaOutput = self.stream.alloc_zeros::<f32>((width * height) as usize).unwrap();
        // self.stream.memcpy_htod(src, dst)
        let mut hostInput = Vec::new();
        for _ in 0..numel {
            hostInput.push(0.);
        }
        let mut hostOutput = hostInput.clone();

        let mut builder = self.stream.launch_builder(&self.compute_kernel);
        builder.arg(&mut cudaOutput);
        builder.arg(&black_hole);
        builder.arg(&accretion_disk);
        builder.arg(&camera);
        builder.arg(&hyperparams);
        builder.arg(&numel);
        unsafe { let _ = builder.launch(LaunchConfig::for_num_elems(numel as u32));};

        let _ =self.stream.memcpy_dtoh(&cudaOutput, &mut hostOutput);

        let mut image =
            Image::gen_image_color(width as u16, height as u16, macroquad::color::BLACK);

        for (index, &value) in hostOutput.iter().enumerate() {
            let px = index as u32 % width as u32;
            let py = index as u32 / width as u32;

            if py < height as u32 {
                image.set_pixel(px, py, macroquad::color::Color { r:value, g:value, b:value, a:value });
            }
        }
        image
    }
}
