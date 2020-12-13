mod network_manager;
mod render;

use bytes::Bytes;
use eyre::Result;
use network_manager::NetworkManager;
use render::Painter;
use winit::window::Window;

const TILE_SIZE: u32 = 256;
const PI: f64 = std::f64::consts::PI;

pub struct Map {
    lng: f64,
    lat: f64,
    zoom: u32,
    painter: Painter,
    nm: NetworkManager,
    tiles: Vec<Vec<Bytes>>,
    width: u32,
    height: u32,
}

impl Map {
    pub async fn new(lng: f64, lat: f64, zoom: u32, window: &Window) -> Result<Self> {
        let painter = Painter::new(window).await?;
        let nm = NetworkManager::new()?;
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        let mut map = Self {
            lng,
            lat,
            zoom,
            painter,
            nm,
            tiles: Vec::new(),
            width,
            height,
        };
        map.load_tiles().await?;
        println!("Map created");
        Ok(map)
    }

    pub async fn render(&mut self) -> Result<()> {
        self.painter.render()?;
        Ok(())
    }

    pub async fn load_tiles(&mut self) -> Result<()> {
        let tile_across = 2u32.pow(self.zoom);
        let world_size = TILE_SIZE * tile_across;
        let mercator_x = world_size as f64 * (self.lng / 360.0 + 0.5);
        let mercator_y =
            world_size as f64 * (1.0 - ((PI * (0.25 + self.lat / 360.0)).tan().ln()) / PI) / 2.0;
        let x0 = (mercator_x - self.width as f64 / 2.0).floor();
        let y0 = (mercator_y - self.height as f64 / 2.0).floor();
        let corner_tile_x = (x0 / TILE_SIZE as f64).floor() as u32;
        let corner_tile_y = (y0 / TILE_SIZE as f64).floor() as u32;
        let tile = self
            .nm
            .load_tile(corner_tile_x, corner_tile_y, self.zoom)
            .await?;
        self.tiles.push(vec![tile]);
        println!("tiles loaded");
        Ok(())
    }

    pub fn set_data(&mut self) -> Result<()> {
        self.painter.load_textures(&self.tiles)?;
        Ok(())
    }
}
