use super::Vertex;

use wgpu::{
    include_spirv, BindGroupLayout, BlendDescriptor, ColorStateDescriptor, ColorWrite, CullMode,
    Device, FrontFace, IndexFormat, PipelineLayoutDescriptor, PrimitiveTopology,
    ProgrammableStageDescriptor, RasterizationStateDescriptor, RenderPipeline,
    RenderPipelineDescriptor, TextureFormat, VertexStateDescriptor,
};

pub(crate) struct Pipeline {
    render_pipeline: RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        texture_bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let vs_module = device.create_shader_module(include_spirv!("shaders/shader.vert.spv"));
        let fs_module = device.create_shader_module(include_spirv!("shaders/shader.frag.spv"));

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            primitive_topology: PrimitiveTopology::TriangleList,
            color_states: &[ColorStateDescriptor {
                format,
                color_blend: BlendDescriptor::REPLACE,
                alpha_blend: BlendDescriptor::REPLACE,
                write_mask: ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self { render_pipeline }
    }

    pub fn get(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}
