mod definitions;
mod create_info;

pub use definitions::*;
pub use create_info::*;

use tuhka::vk;
use leimu_core::slice;
use leimu_mem::{
    alloc::LocalAlloc,
    vec::Vec32,
    vec32,
};

use crate::{
    error::*,
    gpu::prelude::*,
    sync::Arc,
};

/// Contains the handle and metadata of a graphics pipeline.
///
/// This is [`Clone`], [`Send`] and [`Sync`].
#[derive(Clone)]
pub struct GraphicsPipeline {
    handle: PipelineHandle,
    samples: MsaaSamples,
    vertex_input_bindings: Arc<[VertexInputBinding]>,
    color_outputs_and_dynamic_states: Arc<[i32]>,
    n_color_output_formats: u32,
    depth_output_format: Format,
    stencil_output_format: Format,
}

impl GraphicsPipeline {

    #[inline(always)]
    pub(super) unsafe fn new(
        device: Device,
        handle: vk::Pipeline,
        shader_set: ShaderSet,
        create_info: &GraphicsPipelineCreateTemplate,
    ) -> Self {
        let n_color_output_formats = create_info.color_outputs.len() as usize;
        let color_outputs_and_dynamic_states =
        unsafe {
            let n = n_color_output_formats + create_info.dynamic_states.len() as usize;
            let data: Arc<[i32]> = (0..n).map(|_| 0).collect();
            let data = Arc::into_raw(data);
            let ptr = data.cast_mut().as_mut().unwrap_unchecked().as_mut_ptr();
            for (i, (format, _)) in create_info.color_outputs.iter().enumerate() {
                ptr.add(i).write(format.as_raw());
            }
            slice::cast::<_, i32>(&create_info.dynamic_states).unwrap().as_ptr()
                .copy_to_nonoverlapping(
                    ptr.add(n_color_output_formats),
                    create_info.dynamic_states.len() as usize
                );
            Arc::from_raw(data)
        };
        Self {
            handle: unsafe {
                PipelineHandle::new(device, handle, shader_set)
            },
            samples: create_info.sample_shading_info
                .map(|info| info.samples)
                .unwrap_or(MsaaSamples::X1),
            vertex_input_bindings: 
                create_info.vertex_input_bindings
                .iter()
                .cloned()
                .collect()
            ,
            color_outputs_and_dynamic_states,
            n_color_output_formats: n_color_output_formats as u32,
            depth_output_format: create_info.depth_output_format,
            stencil_output_format: create_info.stencil_output_format,
        }
    }

    #[inline(always)]
    pub fn handle(&self) -> &PipelineHandle {
        &self.handle
    }

    #[inline(always)]
    pub fn samples(&self) -> MsaaSamples {
        self.samples
    }

    #[inline(always)]
    pub fn vertex_input_bindings(&self) -> &[VertexInputBinding] {
        &self.vertex_input_bindings
    }

    #[inline(always)]
    pub fn color_output_formats(&self) -> &[Format] {
        unsafe {
            slice::cast(
                &self.color_outputs_and_dynamic_states[0..self.n_color_output_formats as usize]
            ).unwrap()
        }
    }

    #[inline(always)]
    pub fn depth_output_format(&self) -> Format {
        self.depth_output_format
    }

    #[inline(always)]
    pub fn stencil_output_format(&self) -> Format {
        self.stencil_output_format
    }

    #[inline(always)]
    pub fn dynamic_states(&self) -> &[DynamicState] {
        unsafe {
            slice::cast(
                &self.color_outputs_and_dynamic_states[self.n_color_output_formats as usize..]
            ).unwrap()
        }
    }

    #[inline(always)]
    pub fn has_dynamic_state(&self, dynamic_state: DynamicState) -> bool {
        self.dynamic_states().contains(&dynamic_state)
    }
}
