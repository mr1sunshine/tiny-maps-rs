mod painter;
mod network_manager;

use winit::window::Window;
use painter::Painter;
use eyre::Result;
use network_manager::NetworkManager;

pub struct Map {
    lng: f64,
    lat: f64,
    zoom: u32,
    painter: Painter,
    nm: NetworkManager
}

impl Map {
    pub async fn new(lng: f64, lat: f64, zoom: u32, window: &Window) -> Result<Self> {
        let painter = Painter::new(window).await?;
        let nm = NetworkManager::new()?;
        Ok(Self {lng, lat, zoom, painter, nm})
    }

    pub async fn render(&mut self) -> Result<()> {
        Ok(())
    }
}

