pub mod error;
pub mod pipeline;

pub use error::Error;
pub use pipeline::*;

use sdl3::gpu as sdlgpu;
use shame::pipeline_kind::IsPipelineKind;

pub use shame::*;

pub struct Gpu {
    command_buffer: sdlgpu::CommandBuffer,
    device: sdlgpu::Device,
    surface_format: Option<sdlgpu::TextureFormat>,
}

impl core::ops::Deref for Gpu {
    type Target = sdlgpu::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl Gpu {
    pub fn new(
        command_buffer: sdlgpu::CommandBuffer,
        device: sdlgpu::Device,
        surface_format: Option<sdlgpu::TextureFormat>,
    ) -> Self {
        Self {
            command_buffer,
            device,
            surface_format,
        }
    }

    pub fn command_buffer(&self) -> &sdlgpu::CommandBuffer {
        &self.command_buffer
    }

    pub fn surface_format(&self) -> Option<sdlgpu::TextureFormat> {
        self.surface_format
    }

    #[track_caller]
    pub fn create_pipeline_encoder<P: IsPipelineKind>(
        &self,
        desc: PipelineEncoderOptions,
    ) -> Result<PipelineEncoder<P>, Error> {
        Ok(PipelineEncoder {
            gpu: sdlgpu::Device::clone(self),
            enc_guard: start_encoding(Settings {
                lang: Language::Wgsl,
                colored_error_messages: desc.colored_error_messages,
                error_excerpt: desc.error_excerpt,
                shader_identifier_prefix: desc.shader_identifier_prefix,
                vertex_writable_storage_by_default: desc
                    .vertex_writable_storage_by_default,
                zero_init_workgroup_memory: desc.zero_init_workgroup_memory,
            })?,
            surface_format: self.surface_format,
        })
    }
}
