use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    FabricMeta, Game, Intermediary, Loader, LoaderWithIntermediary, META_API, Mapping, Profile,
};

/// An HTTP client for interaction with the Fabric Meta API.
pub struct FabricMetaClient {
    http: Client,
}

impl FabricMetaClient {
    /// Reusing the provided `reqwest::Client`.
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    async fn get_meta<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let path = path.strip_prefix("/").unwrap_or(path);

        let url = META_API.parse::<Url>().unwrap().join(path)?;

        let res = self.http.get(url).send().await?.json().await?;

        Ok(res)
    }

    /// Full database, includes all the data. **Warning:** large JSON.
    pub async fn versions(&self) -> anyhow::Result<FabricMeta> {
        self.get_meta("/v2/versions").await
    }

    /// Lists all of the supported game versions.
    pub async fn game_versions(&self) -> anyhow::Result<Vec<Game>> {
        self.get_meta("/v2/versions/game").await
    }

    /// Lists all of the compatible game versions for yarn.
    pub async fn game_versions_yarn(&self) -> anyhow::Result<Vec<Game>> {
        self.get_meta("/v2/versions/game/yarn").await
    }

    /// Lists all of the compatible game versions for intermediary.
    pub async fn game_versions_intermediary(&self) -> anyhow::Result<Vec<Game>> {
        self.get_meta("/v2/versions/game/intermediary").await
    }

    /// Lists all of the intermediary versions, stable is based of the Minecraft version.
    pub async fn intermediary_versions(&self) -> anyhow::Result<Vec<Intermediary>> {
        self.get_meta("/v2/versions/intermediary").await
    }

    /// Lists all of the intermediary for the provided game version, there will only ever be 1.
    pub async fn game_intermediary_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<[Intermediary; 1]> {
        let path = format!("/v2/versions/intermediary/{game}");

        self.get_meta(&path).await
    }

    /// Lists all of the yarn versions, stable is based on the Minecraft version.
    pub async fn yarn_versions(&self) -> anyhow::Result<Vec<Mapping>> {
        self.get_meta("/v2/versions/yarn").await
    }

    /// Lists all of the yarn versions for the provided game version.
    pub async fn game_yarn_versions(&self, game: &str) -> anyhow::Result<Vec<Mapping>> {
        let path = format!("/v2/versions/yarn/{game}");

        self.get_meta(&path).await
    }

    /// Lists all of the loader versions.
    pub async fn loader_versions(&self) -> anyhow::Result<Vec<Loader>> {
        self.get_meta("/v2/versions/loader").await
    }

    /// This returns a list of all the compatible loader versions for a given version of the game, along with the best version of intermediary to use for that version.
    pub async fn game_loader_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<Vec<LoaderWithIntermediary>> {
        let path = format!("/v2/versions/loader/{game}");

        self.get_meta(&path).await
    }

    /// Returns the JSON file that should be used in the standard Minecraft launcher.
    pub async fn profile(&self, game: &str, loader: &str) -> anyhow::Result<Profile> {
        let path = format!("/v2/versions/loader/{game}/{loader}/profile/json");

        self.get_meta(&path).await
    }
}
