use bytes::Bytes;
use eyre::Result;
use reqwest::{header::HeaderMap, Client, ClientBuilder, RequestBuilder};

#[derive(Debug)]
pub(crate) struct NetworkManager {
    client: Client,
}

impl NetworkManager {
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new().build()?;
        Ok(Self { client })
    }

    pub async fn load_tile(&self, x: u32, y: u32, z: u32) -> Result<Bytes> {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let url = format!("http://tile.osm.org/{}/{}/{}.png", z, x, y);
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "*/*".parse().unwrap());
        headers.insert(
            "User-Agent",
            format!("{}/{}", NAME, VERSION).parse().unwrap(),
        );
        headers.insert("Connection", "keep-alive".parse().unwrap());
        let res = self.client.get(&url).headers(headers).send().await?;
        let body = res.bytes().await?;
        Ok(body)
    }
}
