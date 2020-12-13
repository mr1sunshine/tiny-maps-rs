use super::network_manager::NetworkManager;
use super::render::Painter;
use bytes::Bytes;
use eyre::Result;
use winit::window::Window;

const TILE_SIZE: u32 = 256;
const PI: f64 = std::f64::consts::PI;

pub struct Map {
    lng: f64,
    lat: f64,
    zoom: u32,
    painter: Painter,
    nm: NetworkManager,
    width: u32,
    height: u32,
}

impl Map {
    pub async fn new(lng: f64, lat: f64, zoom: u32, window: &Window) -> Result<Self> {
        let mut painter = Painter::new(window).await?;
        let nm = NetworkManager::new()?;
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        let tiles = Map::load_tiles(zoom, lng, lat, width, height, &nm).await?;
        painter.load_textures(&tiles)?;
        let map = Self {
            lng,
            lat,
            zoom,
            painter,
            nm,
            width,
            height,
        };

        println!("Map created");
        Ok(map)
    }

    pub async fn render(&mut self) -> Result<()> {
        self.painter.render()?;
        Ok(())
    }

    async fn load_tiles(
        zoom: u32,
        lng: f64,
        lat: f64,
        width: u32,
        height: u32,
        nm: &NetworkManager,
    ) -> Result<Vec<Vec<Bytes>>> {
        let tile_across = 2u32.pow(zoom);
        let world_size = TILE_SIZE * tile_across;
        let mercator_x = world_size as f64 * (lng / 360.0 + 0.5);
        let mercator_y =
            world_size as f64 * (1.0 - ((PI * (0.25 + lat / 360.0)).tan().ln()) / PI) / 2.0;
        let x0 = (mercator_x - width as f64 / 2.0).floor();
        let y0 = (mercator_y - height as f64 / 2.0).floor();
        let corner_tile_x = (x0 / TILE_SIZE as f64).floor() as u32;
        let corner_tile_y = (y0 / TILE_SIZE as f64).floor() as u32;
        let tile = nm.load_tile(corner_tile_x, corner_tile_y, zoom).await?;
        println!("tiles loaded");
        Ok(vec![vec![tile]])
    }

    pub fn set_data(&mut self) -> Result<()> {
        // self.painter.load_textures(&self.tiles)?;
        Ok(())
    }
}
