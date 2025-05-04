use super::PipelineEncoder;
use crate::error::Error;

use bytemuck::cast_slice;
use naga::{
    back::spv,
    front::wgsl,
    valid::{Capabilities, ShaderStages, SubgroupOperationSet, ValidationFlags, Validator},
};
use sdl3::gpu as sdlgpu;
use shame::pipeline_kind::Render;
use shame::results::{LanguageCode, RenderPipeline};
use shame::{DrawContext, Indexing};

fn create_shader(
    gpu: &sdlgpu::Device,
    lang_code: LanguageCode,
    shader_stage: sdlgpu::ShaderStage,
) -> Result<sdlgpu::Shader, Error> {
    let code_src = match lang_code {
        LanguageCode::Wgsl(code) => code,
    };
    let module = wgsl::parse_str(&code_src).expect("failed to parse wgsl");
    let module_info = Validator::new(ValidationFlags::all(), Capabilities::all())
        .subgroup_stages(ShaderStages::all())
        .subgroup_operations(SubgroupOperationSet::all())
        .validate(&module)
        .expect("failed to validate wgsl");

    let code = spv::write_vec(&module, &module_info, &spv::Options::default(), None)
        .expect("failed to write spv");

    Ok(gpu
        .create_shader()
        .with_code(
            sdlgpu::ShaderFormat::SpirV,
            cast_slice(code.as_slice()),
            shader_stage,
        )
        .build()
        .expect("failed to build pipeline"))
}

fn render_pipeline(
    gpu: &sdlgpu::Device,
    pdef: RenderPipeline,
    surface_format: Option<sdlgpu::TextureFormat>,
) -> Result<sdlgpu::GraphicsPipeline, Error> {
    enum Shaders {
        Separate(sdlgpu::Shader, sdlgpu::Shader),
        Shared(sdlgpu::Shader),
    }

    let (shaders, v_entry, f_entry) = {
        let v = pdef.shaders.vert_entry_point;
        let f = pdef.shaders.frag_entry_point;
        let shaders = match pdef.shaders.into_shared_shader_code() {
            Ok(shader) => Shaders::Shared(create_shader(gpu, shader, sdlgpu::ShaderStage::Vertex)?),
            Err((vert, frag)) => Shaders::Separate(
                create_shader(gpu, vert, sdlgpu::ShaderStage::Vertex)?,
                create_shader(gpu, frag, sdlgpu::ShaderStage::Fragment)?,
            ),
        };

        (shaders, v, f)
    };

    Ok(gpu.create_graphics_pipeline().with_vertex_shader(shaders))
}

impl PipelineEncoder<Render> {
    #[track_caller]
    #[must_use]
    pub fn new_render_pipeline(&mut self, vertex_indexing: Indexing) -> DrawContext {
        self.enc_guard.new_render_pipeline(vertex_indexing)
    }

    #[track_caller]
    pub fn finish(self) -> Result<sdlgpu::GraphicsPipeline, Error> {
        let pdef = self.enc_guard.finish()?;
        Ok(render_pipeline(&self.gpu, pdef, self.surface_format)?)
    }
}
