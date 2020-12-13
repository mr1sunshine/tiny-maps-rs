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
        let url = format!("http://tile.osm.org/{}/{}/{}.png", z, x, y);
        let mut headers = HeaderMap::new();
        // headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "*/*".parse().unwrap());
        headers.insert("User-Agent", "tiny-maps-rs/0.1.0".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
        // let res = RequestBuilder::new().headers(headers).build()?;
        let res = self.client.get(&url).headers(headers).send().await?;
        println!("{:#?}", res);
        let body = res.bytes().await?;
        Ok(body)
    }
}
