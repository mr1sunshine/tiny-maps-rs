use eyre::Result;
use reqwest::{Client, ClientBuilder};
use bytes::Bytes;

#[derive(Debug)]
pub(crate) struct NetworkManager {
    client: Client,
}

impl NetworkManager {
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new().gzip(true).build()?;
        Ok(Self {
            client,
        })
    }

    pub async fn load_tile(&self, x: u32, y: u32, z: u32) -> Result<Bytes> {
        let url = format!("https://tile.osm.org/{}/{}/{}.png", x, y, z);
        let res = self.client.get(&url).send().await?;
        let body = res.bytes().await?;
        Ok(body)
    }
}
