use super::PipelineEncoder;
use crate::Error;

use std::ffi::CString;

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
    code: &LanguageCode,
    entry: &str,
    gpu: &sdlgpu::Device,
    shader_stage: sdlgpu::ShaderStage,
) -> Result<sdlgpu::Shader, Error> {
    let entry_c = CString::new(entry)?;
    let code_src = match code {
        LanguageCode::Wgsl(code) => code,
    };
    let module = wgsl::parse_str(&code_src)?;
    let module_info =
        Validator::new(ValidationFlags::all(), Capabilities::all())
            .subgroup_stages(ShaderStages::all())
            .subgroup_operations(SubgroupOperationSet::all())
            .validate(&module)?;

    let code =
        spv::write_vec(&module, &module_info, &spv::Options::default(), None)?;

    gpu.create_shader()
        .with_code(
            sdlgpu::ShaderFormat::SpirV,
            cast_slice(code.as_slice()),
            shader_stage,
        )
        .with_entrypoint(&entry_c)
        .build()
        .map_err(|e| Error::SdlError(e))
}

fn render_pipeline(
    gpu: &sdlgpu::Device,
    pdef: RenderPipeline,
    surface_format: Option<sdlgpu::TextureFormat>,
) -> Result<sdlgpu::GraphicsPipeline, Error> {
    let v_shd = create_shader(
        &pdef.shaders.vert_code,
        pdef.shaders.vert_entry_point,
        gpu,
        sdlgpu::ShaderStage::Vertex,
    )?;
    let f_shd = create_shader(
        &pdef.shaders.frag_code,
        pdef.shaders.frag_entry_point,
        gpu,
        sdlgpu::ShaderStage::Fragment,
    )?;
    let mut target_info = sdlgpu::GraphicsPipelineTargetInfo::new();

    if let Some(texture_format) = surface_format {
        target_info = target_info.with_color_target_descriptions(&[
            sdlgpu::ColorTargetDescription::new().with_format(texture_format),
        ]);
    }

    let ds_state = if let Some(depth_stencil) = pdef.pipeline.depth_stencil {
        use sdlgpu::CompareOp;
        use shame::Test;
        let compare_op = match depth_stencil.depth_compare {
            Test::Never => CompareOp::Never,
            Test::Less => CompareOp::Less,
            Test::Equal => CompareOp::Equal,
            Test::LessEqual => CompareOp::LessOrEqual,
            Test::Greater => CompareOp::Greater,
            Test::NotEqual => CompareOp::NotEqual,
            Test::GreaterEqual => CompareOp::GreaterOrEqual,
            Test::Always => CompareOp::Always,
        };
        sdlgpu::DepthStencilState::new()
            .with_compare_op(compare_op)
            .with_enable_depth_test(true)
            .with_enable_depth_write(depth_stencil.depth_write_enabled)
            .with_write_mask(depth_stencil.stencil.w_mask as u8)
    } else {
        sdlgpu::DepthStencilState::default()
    };

    gpu.create_graphics_pipeline()
        .with_depth_stencil_state(ds_state)
        .with_vertex_shader(&v_shd)
        .with_fragment_shader(&f_shd)
        .with_primitive_type(sdlgpu::PrimitiveType::TriangleList)
        .with_fill_mode(sdlgpu::FillMode::Fill)
        .with_target_info(target_info)
        .build()
        .map_err(|e| Error::SdlError(e))
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
        render_pipeline(&self.gpu, pdef, self.surface_format)
    }
}
