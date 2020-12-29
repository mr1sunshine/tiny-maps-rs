use crate::{tile_coordinates::TileCoordinates, utils::Rect};
use bytes::Bytes;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Tile {
    x: f32,
    y: f32,
    _z: f32,
    #[derivative(Debug = "ignore")]
    data: Bytes,
    coords: TileCoordinates,
}

impl Tile {
    pub fn new(x: f32, y: f32, z: f32, data: Bytes, coords: TileCoordinates) -> Tile {
        Self {
            x,
            y,
            _z: z,
            data,
            coords,
        }
    }

    pub fn coords(&self) -> &TileCoordinates {
        &self.coords
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
