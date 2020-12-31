use std::time::Instant;

use super::{texture::Texture, vertex::Vertex};
use crate::tile::Tile;
use eyre::Result;
use log::debug;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Buffer,
    BufferUsage, Device, Queue,
};

const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];

pub(crate) struct Grid {
    pub bind_groups: Vec<BindGroup>,
    pub textures: Vec<Texture>,
    pub vertex_buffers: Vec<Buffer>,
    pub index_buffer: Buffer,
    pub num_indices: u32,
}

impl Grid {
    pub fn new(
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
        tiles: &[Tile],
    ) -> Result<Self> {
        let now = Instant::now();
        let mut bind_groups = Vec::new();
        let mut textures = Vec::new();
        let mut vertex_buffers = Vec::new();

        for tile in tiles {
            // if tile.x() != 18654 || tile.y() != 9481 {
            //     continue;
            // }
            // info!("tile {:#?}", tile);
            let (texture, bind_group) =
                Grid::create_texture_and_bind_group(device, queue, bind_group_layout, tile)?;
            bind_groups.push(bind_group);
            textures.push(texture);
            let coords = tile.coords();
            let vertices = vec![
                Vertex {
                    position: [coords.shader_coords.2, coords.shader_coords.1, 0.0],
                    tex_coords: [coords.texture_coords.2, coords.texture_coords.1],
                },
                Vertex {
                    position: [coords.shader_coords.0, coords.shader_coords.1, 0.0],
                    tex_coords: [coords.texture_coords.0, coords.texture_coords.1],
                },
                Vertex {
                    position: [coords.shader_coords.0, coords.shader_coords.3, 0.0],
                    tex_coords: [coords.texture_coords.0, coords.texture_coords.3],
                },
                Vertex {
                    position: [coords.shader_coords.2, coords.shader_coords.3, 0.0],
                    tex_coords: [coords.texture_coords.2, coords.texture_coords.3],
                },
                // Vertex::new(&tile.right_top()),    // B
                // Vertex::new(&tile.left_top()),     // A
                // Vertex::new(&tile.left_bottom()),  // C
                // Vertex::new(&tile.right_bottom()), // D
            ];
            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: BufferUsage::VERTEX,
            });
            vertex_buffers.push(vertex_buffer);
            // break;
        }
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsage::INDEX,
        });
        let num_indices = INDICES.len() as u32;
        debug!("New grid took {} ms", now.elapsed().as_millis());
        Ok(Self {
            bind_groups,
            textures,
            vertex_buffers,
            index_buffer,
            num_indices,
        })
    }

    fn create_texture_and_bind_group(
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
        tile: &Tile,
    ) -> Result<(Texture, BindGroup)> {
        let texture = Texture::from_bytes(device, queue, tile.data())?;

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        Ok((texture, bind_group))
    }
}
