use crate::{tile_coordinates::TileCoordinates, tile_id::TileId};
use bytes::Bytes;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Tile {
    id: TileId,
    #[derivative(Debug = "ignore")]
    data: Bytes,
    coords: TileCoordinates,
}

impl Tile {
    pub fn new(id: &TileId, data: Bytes, coords: &TileCoordinates) -> Tile {
        Self {
            id: id.clone(),
            data,
            coords: coords.clone(),
        }
    }

    pub fn coords(&self) -> &TileCoordinates {
        &self.coords
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }
}
