use super::PipelineEncoder;
use crate::error::Error;

use sdl3::gpu as sdlgpu;
use shame::pipeline_kind::Compute;
use shame::{DispatchContext, GridSize};

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
        let _pdef = self.enc_guard.finish()?;

        todo!()
    }
}
