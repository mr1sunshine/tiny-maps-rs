use super::network_manager::NetworkManager;
use super::render::Painter;
use crate::tile::Tile;
use bytes::Bytes;
use eyre::Result;
use futures::{future::try_join_all, FutureExt};
use geo::Point;
use log::info;
use winit::window::Window;

const TILE_SIZE: u32 = 1024;
const PI: f64 = std::f64::consts::PI;

pub struct Map {
    point: Point<f64>,
    zoom: u32,
    painter: Painter,
    nm: NetworkManager,
    width: u32,
    height: u32,
    window: Window,
}

impl Map {
    pub async fn new(point: &Point<f64>, zoom: u32, window: Window) -> Result<Self> {
        let nm = NetworkManager::new()?;
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        info!("w: {} height: {}", width, height);

        let tiles = Map::load_tiles(zoom, point, width, height, &nm).await?;
        let painter = Painter::new(&window, &tiles).await?;

        let map = Self {
            point: *point,
            zoom,
            painter,
            nm,
            width,
            height,
            window,
        };

        info!("Map created");
        Ok(map)
    }

    pub async fn render(&mut self) -> Result<()> {
        self.painter.render()?;
        Ok(())
    }

    fn create_tile(
        left: f64,
        top: f64,
        x: u32,
        y: u32,
    ) -> impl FnOnce(Result<Bytes>) -> Result<Tile> {
        move |data| Ok(Tile::new(data?, left, top, x, y))
    }

    async fn load_tiles(
        zoom: u32,
        point: &Point<f64>,
        width: u32,
        height: u32,
        nm: &NetworkManager,
    ) -> Result<Vec<Tile>> {
        let tile_across = 2u32.pow(zoom);
        let world_size = TILE_SIZE * tile_across;
        let mercator_x = world_size as f64 * (point.lng() / 360.0 + 0.5);
        let mercator_y =
            world_size as f64 * (1.0 - ((PI * (0.25 + point.lat() / 360.0)).tan().ln()) / PI) / 2.0;
        let x0 = (mercator_x - width as f64 / 2.0).floor();
        let y0 = (mercator_y - height as f64 / 2.0).floor();
        let corner_tile_x = (x0 / TILE_SIZE as f64).floor() as u32;
        let corner_tile_y = (y0 / TILE_SIZE as f64).floor() as u32;
        let mut futures = Vec::new();
        let mut tile_x = corner_tile_x;
        let mut tile_y = corner_tile_y;
        while ((tile_y * TILE_SIZE) as f64) < y0 + height as f64 {
            while ((tile_x * TILE_SIZE) as f64) < x0 + width as f64 {
                let left = (tile_x * TILE_SIZE) as f64 - x0;
                let top = (tile_y * TILE_SIZE) as f64 - y0;
                futures.push(
                    nm.load_tile(tile_x, tile_y, zoom)
                        .map(Map::create_tile(left, top, tile_x, tile_y)),
                );
                tile_x += 1;
            }
            tile_y += 1;
            tile_x = corner_tile_x;
        }
        let tiles = try_join_all(futures).await?;
        Ok(tiles)
    }

    pub fn zoom(&self) -> u32 {
        self.zoom
    }

    pub async fn set_zoom(&mut self, zoom: u32) -> Result<()> {
        self.zoom = zoom;
        self.update().await?;
        Ok(())
    }

    pub fn point(&self) -> Point<f64> {
        self.point
    }

    pub async fn set_point(&mut self, point: Point<f64>) -> Result<()> {
        self.point = point;
        self.update().await?;
        Ok(())
    }

    async fn update(&mut self) -> Result<()> {
        let tiles =
            Map::load_tiles(self.zoom, &self.point, self.width, self.height, &self.nm).await?;

        self.painter.load_textures(&tiles)?;

        self.window.request_redraw();
        Ok(())
    }
}
