use super::{grid::Grid, Pipeline};
use crate::tile::Tile;
use eyre::Result;
use wgpu::{
    BackendBit, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, LoadOp,
    Operations, PowerPreference, PresentMode, Queue, RenderPassColorAttachmentDescriptor,
    RenderPassDescriptor, RequestAdapterOptions, ShaderStage, Surface, SwapChain,
    SwapChainDescriptor, TextureComponentType, TextureFormat, TextureUsage, TextureViewDimension,
};
use winit::window::Window;

pub(crate) struct Painter {
    device: Device,
    queue: Queue,
    swap_chain: SwapChain,
    sc_desc: SwapChainDescriptor,
    surface: Surface,
    pipeline: Pipeline,
    bind_group_layout: BindGroupLayout,
    grid: Grid,
}

impl Painter {
    pub async fn new(window: &Window, tiles: &[Tile]) -> Result<Self> {
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await?;

        let sc_desc = SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::SampledTexture {
                        multisampled: false,
                        dimension: TextureViewDimension::D2,
                        component_type: TextureComponentType::Uint,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler { comparison: false },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let grid = Grid::new(&device, &queue, &bind_group_layout, tiles)?;
        let pipeline = Pipeline::new(&device, TextureFormat::Bgra8UnormSrgb, &bind_group_layout);

        Ok(Self {
            device,
            queue,
            swap_chain,
            surface,
            sc_desc,
            pipeline,
            bind_group_layout,
            grid,
        })
    }

    pub fn load_textures(&mut self, tiles: &[Tile]) -> Result<()> {
        self.grid = Grid::new(&self.device, &self.queue, &self.bind_group_layout, tiles)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        let frame = match self.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
                self.swap_chain.get_current_frame()?
            }
        };

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(self.pipeline.get());
            for (index, bind_group) in self.grid.bind_groups.iter().enumerate() {
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.grid.vertex_buffers[index].slice(..));
                render_pass.set_index_buffer(self.grid.index_buffer.slice(..));
                render_pass.draw_indexed(0..self.grid.num_indices, 0, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
