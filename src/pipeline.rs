mod compute;
mod render;

use sdl3::gpu as sdlgpu;
use shame::EncodingGuard;
use shame::pipeline_kind::IsPipelineKind;

pub struct PipelineEncoder<P: IsPipelineKind> {
    pub(crate) gpu: sdlgpu::Device,
    pub(crate) enc_guard: EncodingGuard<P>,
    pub(crate) surface_format: Option<sdlgpu::TextureFormat>,
}

/// This struct maps to [`shame::Settings`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PipelineEncoderOptions {
    pub colored_error_messages: bool,
    pub error_excerpt: bool,
    pub shader_identifier_prefix: &'static str,
    pub vertex_writable_storage_by_default: bool,
    pub zero_init_workgroup_memory: bool,
}

impl Default for PipelineEncoderOptions {
    fn default() -> Self {
        Self {
            colored_error_messages: true,
            error_excerpt: true,
            shader_identifier_prefix: "s_",
            vertex_writable_storage_by_default: false,
            zero_init_workgroup_memory: false,
        }
    }
}
