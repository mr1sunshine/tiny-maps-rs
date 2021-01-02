use super::network_manager::NetworkManager;
use super::render::Painter;
use crate::{tile::Tile, tile_coordinates::TileCoordinates, tile_id::TileId};
use bytes::Bytes;
use eyre::Result;
use futures::{future::try_join_all, FutureExt};
use geo::Point;
use log::{debug, info};
use std::time::Instant;
use winit::{dpi::PhysicalSize, window::Window};

const TILE_SIZE: f32 = 256.0;
const PI: f32 = std::f64::consts::PI as f32;

pub struct Map {
    point: Point<f32>,
    zoom: f32,
    painter: Painter,
    nm: NetworkManager,
    width: f32,
    height: f32,
    window: Window,
}

struct TileInfo {
    pub id: TileId,
    pub coords: TileCoordinates,
}

impl Map {
    pub async fn new(point: &Point<f32>, zoom: u32, window: Window) -> Result<Self> {
        let nm = NetworkManager::new()?;
        let scale_factor = window.scale_factor() as f32;
        let PhysicalSize { width, height } = window.inner_size();
        let width = width as f32 / scale_factor;
        let height = height as f32 / scale_factor;

        let zoom = zoom as f32;
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
        let now = Instant::now();

        self.painter.render()?;
        debug!("Render took {} ms", now.elapsed().as_millis());

        Ok(())
    }

    fn create_tile<'a>(
        id: &'a TileId,
        coords: &'a TileCoordinates,
    ) -> impl FnOnce(Result<Bytes>) -> Result<Tile> + 'a {
        move |data| Ok(Tile::new(id, data?, coords))
    }

    async fn load_tiles(
        zoom: f32,
        point: &Point<f32>,
        width: f32,
        height: f32,
        nm: &NetworkManager,
    ) -> Result<Vec<Tile>> {
        let now = Instant::now();
        let required_tiles = Map::create_required_tile_infos(zoom, point, width, height);

        let mut futures = Vec::new();
        for tile in &required_tiles {
            futures.push(
                nm.load_tile(&tile.id)
                    .map(Map::create_tile(&tile.id, &tile.coords)),
            );
        }

        let tiles = try_join_all(futures).await?;
        debug!("Tile loading took {} ms", now.elapsed().as_millis());
        Ok(tiles)
    }

    pub fn zoom(&self) -> u32 {
        self.zoom as u32
    }

    pub async fn set_zoom(&mut self, zoom: u32) -> Result<()> {
        self.zoom = zoom as f32;
        self.update().await?;
        Ok(())
    }

    pub fn point(&self) -> Point<f32> {
        self.point
    }

    pub async fn set_point(&mut self, point: Point<f32>) -> Result<()> {
        self.point = point;
        self.update().await?;
        Ok(())
    }

    async fn update(&mut self) -> Result<()> {
        let now = Instant::now();
        let tiles =
            Map::load_tiles(self.zoom, &self.point, self.width, self.height, &self.nm).await?;

        self.painter.load_textures(&tiles)?;

        self.window.request_redraw();
        debug!("Update took {} ms", now.elapsed().as_millis());
        Ok(())
    }

    fn get_corner_info(
        zoom: f32,
        point: &Point<f32>,
        width: f32,
        height: f32,
    ) -> (f32, f32, TileId) {
        let tile_across = 2f32.powf(zoom);
        let world_size = TILE_SIZE * tile_across;
        let mercator_x = world_size * (point.lng() / 360.0 + 0.5);
        let mercator_y =
            world_size * (1.0 - ((PI * (0.25 + point.lat() / 360.0)).tan().ln()) / PI) / 2.0;
        let x0 = (mercator_x - width / 2.0).floor();
        let y0 = (mercator_y - height / 2.0).floor();
        let tile_x = (x0 / TILE_SIZE).floor();
        let tile_y = (y0 / TILE_SIZE).floor();
        (x0, y0, TileId::new(tile_x, tile_y, zoom))
    }

    fn create_required_tile_infos(
        zoom: f32,
        point: &Point<f32>,
        width: f32,
        height: f32,
    ) -> Vec<TileInfo> {
        let (x0, y0, corner_tile_id) = Map::get_corner_info(zoom, point, width, height);
        let mut tiles = Vec::new();

        let mut tile_x = corner_tile_id.x;
        let mut tile_y = corner_tile_id.y;

        while (tile_y * TILE_SIZE) < (y0 + height) {
            while (tile_x * TILE_SIZE) < (x0 + width) {
                let left = tile_x * TILE_SIZE - x0;
                let top = tile_y * TILE_SIZE - y0;
                let coords = TileCoordinates::new(left, top, width, height, TILE_SIZE);
                let id = TileId::new(tile_x, tile_y, zoom);
                tiles.push(TileInfo { id, coords });
                tile_x += 1.0;
            }
            tile_y += 1.0;
            tile_x = corner_tile_id.x;
        }

        tiles
    }
}
