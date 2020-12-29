use crate::{tile_coordinates::TileCoordinates, utils::Rect};
use bytes::Bytes;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Tile {
    x: u32,
    y: u32,
    _z: u32,
    #[derivative(Debug = "ignore")]
    data: Bytes,
    coords: TileCoordinates,
}

impl Tile {
    pub fn new(x: u32, y: u32, z: u32, data: Bytes, coords: TileCoordinates) -> Tile {
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

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
