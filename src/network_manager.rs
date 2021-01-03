use crate::tile_id::TileId;
use bytes::Bytes;
use eyre::Result;
use hyper::{client::HttpConnector, Body, Client, Method, Request};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct NetworkManager {
    client: Client<HttpConnector>,
}

impl NetworkManager {
    pub fn new() -> Result<Self> {
        let client = Client::new();
        Ok(Self { client })
    }

    pub async fn load_tile(&self, id: &TileId) -> Result<(TileId, Arc<Bytes>)> {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let url = format!("http://tile.osm.org/{}/{}/{}.png", id.z(), id.x(), id.y());
        let user_agent = format!("{}/{}", NAME, VERSION);

        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("User-Agent", user_agent)
            .body(Body::empty())?;

        let res = self.client.request(req).await?;
        let body = Arc::new(hyper::body::to_bytes(res.into_body()).await?);
        Ok((id.clone(), body))
    }
}
