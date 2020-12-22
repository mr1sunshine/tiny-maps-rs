use bytes::Bytes;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Tile {
    #[derivative(Debug = "ignore")]
    data: Bytes,
    top: f64,
    left: f64,
    x: u32,
    y: u32,
}

impl Tile {
    pub fn new(data: Bytes, top: f64, left: f64, x: u32, y: u32) -> Tile {
        Self {
            data,
            top,
            left,
            x,
            y,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
