use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    FabricMeta, Game, Intermediary, Loader, LoaderWithIntermediary, META_API, Mapping, Profile,
};

pub struct FabricMetaClient {
    http: Client,
}

impl FabricMetaClient {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    async fn get_meta<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let path = path.strip_prefix("/").unwrap_or(path);

        let url = META_API.parse::<Url>().unwrap().join(path)?;

        let res = self.http.get(url).send().await?.json().await?;

        Ok(res)
    }

    pub async fn versions(&self) -> anyhow::Result<FabricMeta> {
        self.get_meta("/v2/versions").await
    }

    pub async fn game_versions(&self) -> anyhow::Result<Vec<Game>> {
        self.get_meta("/v2/versions/game").await
    }

    pub async fn game_versions_yarn(&self) -> anyhow::Result<Vec<Game>> {
        self.get_meta("/v2/versions/game/yarn").await
    }

    pub async fn game_versions_intermediary(&self) -> anyhow::Result<Vec<Game>> {
        self.get_meta("/v2/versions/game/intermediary").await
    }

    pub async fn intermediary_versions(&self) -> anyhow::Result<Vec<Intermediary>> {
        self.get_meta("/v2/versions/intermediary").await
    }

    pub async fn game_intermediary_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<Vec<Intermediary>> {
        let path = format!("/v2/versions/intermediary/{game}");

        self.get_meta(&path).await
    }

    pub async fn yarn_versions(&self) -> anyhow::Result<Vec<Mapping>> {
        self.get_meta("/v2/versions/yarn").await
    }

    pub async fn game_yarn_versions(&self, game: &str) -> anyhow::Result<Vec<Mapping>> {
        let path = format!("/v2/versions/yarn/{game}");

        self.get_meta(&path).await
    }

    pub async fn loader_versions(&self) -> anyhow::Result<Vec<Loader>> {
        self.get_meta("/v2/versions/loader").await
    }

    pub async fn game_loader_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<Vec<LoaderWithIntermediary>> {
        let path = format!("/v2/versions/loader/{game}");

        self.get_meta(&path).await
    }

    pub async fn profile(&self, game: &str, loader: &str) -> anyhow::Result<Profile> {
        let path = format!("/v2/versions/loader/{game}/{loader}/profile/json");

        self.get_meta(&path).await
    }
}
