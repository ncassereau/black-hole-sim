use cudarc::driver::{
    CudaContext, CudaFunction, CudaSlice, CudaStream, DevicePtr, LaunchConfig, PushKernelArg,
};
use cudarc::nvrtc::Ptx;
use macroquad::texture::Image;
use std::error::Error;
use std::sync::Arc;

use crate::black_hole::AccretionDisk;
use crate::scene::Camera;
use crate::{BLOCK_SIZE, Backend};
use crate::{
    BlackHole, CUDAAccretionDisk, CUDABlackHole, CUDACamera, CUDAHyperparameters, CUDASkybox,
    Hyperparameters, Scene, Skybox,
};

struct OutputBuffer {
    numel: usize,
    device_buffer: CudaSlice<f32>,
    host_buffer: Vec<f32>,
}

impl OutputBuffer {
    pub fn new(stream: Arc<CudaStream>, numel: usize) -> Result<Self, Box<dyn Error>> {
        let device_buffer = stream.alloc_zeros::<f32>(numel)?;

        let host_buffer = vec![0.0; numel];

        Ok(Self {
            numel,
            device_buffer,
            host_buffer,
        })
    }
}

pub struct CUDABackend {
    stream: Arc<CudaStream>,
    compute_kernel: CudaFunction,
    skybox_cuda_buffer: Option<CudaSlice<f32>>,
    output_buffer: Option<OutputBuffer>,
}

impl CUDABackend {
    fn get_dim(numel: u32, block_size: u32) -> u32 {
        (numel + block_size - 1) / block_size
    }

    fn stream(&self) -> Arc<CudaStream> {
        Arc::clone(&self.stream)
    }

    fn get_skybox_ptr(&mut self, skybox: &Arc<Skybox>) -> *mut f32 {
        if self.skybox_cuda_buffer.is_none() {
            let buffer = self.stream.memcpy_stod(skybox.as_f32_slice()).unwrap();
            self.skybox_cuda_buffer = Some(buffer);
        }

        let buffer = self.skybox_cuda_buffer.as_ref().unwrap();
        let stream_handle = self.stream();
        buffer.device_ptr(&stream_handle).0 as *mut f32
    }

    fn ensure_output_buffer(&mut self, numel: usize) -> Result<(), Box<dyn Error>> {
        if self.output_buffer.is_none() || self.output_buffer.as_ref().unwrap().numel != numel {
            self.output_buffer = Some(OutputBuffer::new(self.stream(), numel)?);
        }
        Ok(())
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
            skybox_cuda_buffer: None,
            output_buffer: None,
        })
    }

    fn compute(
        &mut self,
        accretion_disk: &AccretionDisk,
        black_hole: &BlackHole,
        skybox: Arc<Skybox>,
        camera: &Camera,
        scene: &Scene,
        hyperparams: &Hyperparameters,
    ) -> Result<Image, Box<dyn Error>> {
        let hyperparams: CUDAHyperparameters = hyperparams.into();
        let black_hole: CUDABlackHole = black_hole.into();
        let accretion_disk: CUDAAccretionDisk = accretion_disk.into();
        let skybox = CUDASkybox::new(
            self.get_skybox_ptr(&skybox),
            skybox.width(),
            skybox.height(),
        );
        let camera = CUDACamera::from_camera_scene(camera, scene);
        let (width, height) = scene.screen_size().unpack();
        let numel = 3 * (width * height) as usize;

        self.ensure_output_buffer(numel)?;

        let mut builder = self.stream.launch_builder(&self.compute_kernel);
        builder.arg(&mut self.output_buffer.as_mut().unwrap().device_buffer);
        builder.arg(&black_hole);
        builder.arg(&accretion_disk);
        builder.arg(&skybox);
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

        let output_buffer = self.output_buffer.as_mut().unwrap();
        self.stream
            .memcpy_dtoh(&output_buffer.device_buffer, &mut output_buffer.host_buffer)?;

        Self::to_image(&output_buffer.host_buffer, width as u16, height as u16)
    }
}
