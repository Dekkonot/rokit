use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub assets: Vec<Asset>,
    pub tag_name: String,
    pub prerelease: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    pub id: u64,
    pub url: Url,
    pub name: String,
}
