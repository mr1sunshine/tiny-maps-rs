use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub(crate) struct TileId {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TileId {
    pub fn new(x: f32, y: f32, z: f32) -> TileId {
        Self { x, y, z }
    }

    pub fn x(&self) -> u32 {
        self.x as u32
    }

    pub fn y(&self) -> u32 {
        self.y as u32
    }

    pub fn z(&self) -> u32 {
        self.z as u32
    }
}

impl Hash for TileId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let id = format!("{}/{}/{}", self.z(), self.x(), self.y());
        id.hash(state);
    }
}

impl PartialEq for TileId {
    fn eq(&self, other: &TileId) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

impl Eq for TileId {}
