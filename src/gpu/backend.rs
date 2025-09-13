use macroquad::texture::Image;
use std::fs;
use std::num::NonZeroU64;
use wgpu::util::DeviceExt;
use wgpu::{Device, Queue};
use wgpu::{DownlevelFlags, Instance, InstanceDescriptor, ShaderModule};

use super::types::GPUBlackHole;
use crate::black_hole::AccretionDisk;
use crate::gpu::GPUBackendError;
use crate::gpu::types::{GPUAccretionDisk, GPUCamera, GPUHyperparameters};
use crate::scene::Camera;
use crate::{BlackHole, Scene};

fn get_shaders(device: &Device) -> ShaderModule {
    // Get shaders folders
    let path = std::path::Path::new(file!())
        .parent()
        .unwrap()
        .join("shaders");

    // Find all wgsl files within folder
    let wgsl_files: Vec<_> = path
        .read_dir()
        .expect("No shaders folder")
        .filter(|e| e.as_ref().unwrap().path().extension().unwrap_or_default() == "wgsl")
        .collect();

    // Concatenate all shaders files
    let combined_shader = wgsl_files
        .iter()
        .map(|entry| {
            fs::read_to_string(entry.as_ref().unwrap().path()).expect("Failed to read shader file")
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    // Create shader module from massive shader
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Combined"),
        source: wgpu::ShaderSource::Wgsl(combined_shader.into()),
    })
}

macro_rules! binding_size {
    ($t:ty) => {
        Some(NonZeroU64::new(std::mem::size_of::<$t>() as u64).unwrap())
    };
}

#[derive(Debug)]
pub struct GPUBackend {
    device: Device,
    queue: Queue,
    module: ShaderModule,
}

impl GPUBackend {
    pub async fn new() -> Result<Self, GPUBackendError> {
        let instance = Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::DX12 | wgpu::Backends::METAL,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        log::info!("Running on Adapter: {:#?}", adapter.get_info());

        let downlevel_capabilities = adapter.get_downlevel_capabilities();
        if !downlevel_capabilities
            .flags
            .contains(DownlevelFlags::COMPUTE_SHADERS)
        {
            return Err(GPUBackendError::NoComputeShaders);
        }

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::SHADER_F64 | wgpu::Features::SHADER_INT64,
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await?;

        let module = get_shaders(&device);

        Ok(Self {
            device,
            queue,
            module,
        })
    }

    fn to_image(result: &[[f32; 4]], width: u16, height: u16) -> Image {
        let mut image = Image::gen_image_color(width, height, macroquad::color::BLACK);

        for (index, &[r, g, b, a]) in result.iter().enumerate() {
            let px = index as u32 % width as u32;
            let py = index as u32 / width as u32;

            if py < height as u32 {
                image.set_pixel(px, py, macroquad::color::Color { r, g, b, a });
            }
        }
        image
    }

    pub fn compute(
        &self,
        accretion_disk: &AccretionDisk,
        black_hole: &BlackHole,
        camera: &Camera,
        scene: &Scene,
        dλ0: f64,
        bounding_box_radius: f64,
        num_integration_steps: usize,
        normalization_interval: usize,
        integration_error_tolerance: f64,
        min_dλ: f64,
        max_dλ: f64,
        max_retries: usize,
    ) -> Image {
        let black_hole: GPUBlackHole = black_hole.into();
        let accretion_disk: GPUAccretionDisk = accretion_disk.into();
        let camera = GPUCamera::from_camera_scene(camera, scene);
        let hyperparams = GPUHyperparameters::new(
            dλ0,
            bounding_box_radius,
            num_integration_steps,
            normalization_interval,
            integration_error_tolerance,
            min_dλ,
            max_dλ,
            max_retries,
        );

        let num_pixels = (camera.screen_width * camera.screen_height) as u32;

        let black_hole_data_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[black_hole]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
        let camera_data_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[camera]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
        let accretion_disk_data_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[accretion_disk]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
        let hyperparams_data_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[hyperparams]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });

        let output_data_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (num_pixels * std::mem::size_of::<[f32; 4]>() as u32) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let download_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: output_data_buffer.size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        // Input buffer
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: binding_size!(GPUBlackHole),
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: binding_size!(GPUCamera),
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: binding_size!(GPUAccretionDisk),
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: binding_size!(GPUHyperparameters),
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        // Output buffer
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: black_hole_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: camera_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: accretion_disk_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: hyperparams_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: output_data_buffer.as_entire_binding(),
                },
            ],
        });
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                module: &self.module,
                entry_point: Some("compute_image"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(num_pixels.div_ceil(64), 1, 1);
        drop(compute_pass);
        encoder.copy_buffer_to_buffer(
            &output_data_buffer,
            0,
            &download_buffer,
            0,
            output_data_buffer.size(),
        );
        let command_buffer = encoder.finish();
        self.queue.submit([command_buffer]);
        let buffer_slice = download_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        self.device.poll(wgpu::PollType::Wait).unwrap();
        let data = buffer_slice.get_mapped_range();
        let result: &[[f32; 4]] = bytemuck::cast_slice(&data);

        Self::to_image(
            result,
            camera.screen_width as u16,
            camera.screen_height as u16,
        )
    }
}
