use super::PipelineEncoder;
use crate::error::Error;

use bytemuck::cast_slice;
use naga::{
    back::spv,
    front::wgsl,
    valid::{
        Capabilities, ShaderStages, SubgroupOperationSet, ValidationFlags,
        Validator,
    },
};
use sdl3::gpu as sdlgpu;
use shame::pipeline_kind::Render;
use shame::results::{LanguageCode, RenderPipeline};
use shame::{DrawContext, Indexing};

fn create_shader(
    gpu: &sdlgpu::Device,
    lang_code: &LanguageCode,
    shader_stage: sdlgpu::ShaderStage,
) -> Result<sdlgpu::Shader, Error> {
    let code_src = match lang_code {
        LanguageCode::Wgsl(code) => code,
    };
    let module = wgsl::parse_str(&code_src).expect("failed to parse wgsl");
    let module_info =
        Validator::new(ValidationFlags::all(), Capabilities::all())
            .subgroup_stages(ShaderStages::all())
            .subgroup_operations(SubgroupOperationSet::all())
            .validate(&module)
            .expect("failed to validate wgsl");

    let code =
        spv::write_vec(&module, &module_info, &spv::Options::default(), None)
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
    let v_shd = create_shader(
        gpu,
        &pdef.shaders.vert_code,
        sdlgpu::ShaderStage::Vertex,
    )
    .expect("failed to compile vert shader");
    let f_shd = create_shader(
        gpu,
        &pdef.shaders.frag_code,
        sdlgpu::ShaderStage::Vertex,
    )
    .expect("failed to compile frag shader");

    let pipeline = gpu
        .create_graphics_pipeline()
        .with_vertex_shader(&v_shd)
        .with_fragment_shader(&f_shd)
        .build()
        .expect("failed to build pipeline");

    Ok(pipeline)
}

impl PipelineEncoder<Render> {
    #[track_caller]
    #[must_use]
    pub fn new_render_pipeline(
        &mut self,
        vertex_indexing: Indexing,
    ) -> DrawContext {
        self.enc_guard.new_render_pipeline(vertex_indexing)
    }

    #[track_caller]
    pub fn finish(self) -> Result<sdlgpu::GraphicsPipeline, Error> {
        let pdef = self.enc_guard.finish()?;
        Ok(render_pipeline(&self.gpu, pdef, self.surface_format)?)
    }
}
