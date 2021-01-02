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
