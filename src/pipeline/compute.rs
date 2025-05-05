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
use shame::pipeline_kind::Compute;
use shame::results::{ComputePipeline, LanguageCode};
use shame::{DispatchContext, GridSize};

fn code_to_spv(code: &LanguageCode) -> Result<Vec<u32>, Error> {
    let code_src = match code {
        LanguageCode::Wgsl(code) => code,
    };
    let module = wgsl::parse_str(&code_src)?;
    let module_info =
        Validator::new(ValidationFlags::all(), Capabilities::all())
            .subgroup_stages(ShaderStages::all())
            .subgroup_operations(SubgroupOperationSet::all())
            .validate(&module)?;

    spv::write_vec(&module, &module_info, &spv::Options::default(), None)
        .map_err(|e| Error::SpvError(e))
}

fn compute_pipeline(
    gpu: &sdlgpu::Device,
    pdef: ComputePipeline,
) -> Result<sdlgpu::ComputePipeline, Error> {
    let code = code_to_spv(&pdef.shader.code)?;
    let entry_c = CString::new(pdef.shader.entry_point)?;
    let [x, y, z] = pdef.pipeline.grid_info.thread_grid_size_per_workgroup;

    gpu.create_compute_pipeline()
        .with_code(sdlgpu::ShaderFormat::SpirV, cast_slice(code.as_slice()))
        .with_entrypoint(&entry_c)
        .with_thread_count(x, y, z)
        .build()
        .map_err(|e| Error::SdlError(e))
}

impl PipelineEncoder<Compute> {
    #[track_caller]
    #[must_use]
    pub fn new_compute_pipeline<const N: usize>(
        &mut self,
        thread_grid_size_per_workgroup: [u32; N],
    ) -> DispatchContext<<[u32; N] as GridSize>::Dim>
    where
        [u32; N]: GridSize,
    {
        self.enc_guard
            .new_compute_pipeline(thread_grid_size_per_workgroup)
    }

    #[track_caller]
    pub fn finish(self) -> Result<sdlgpu::ComputePipeline, Error> {
        let pdef = self.enc_guard.finish()?;
        compute_pipeline(&self.gpu, pdef)
    }
}
