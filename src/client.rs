use std::sync::Arc;

use dashmap::{DashMap, mapref::one::Ref};
use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    FabricMeta, Game, Intermediary, Loader, LoaderWithIntermediary, META_API, Mapping, Profile,
    Server,
};

/// An HTTP client for interaction with the Fabric Meta API.
#[derive(Clone)]
pub struct FabricMetaClient {
    http: Client,
    cache: Arc<Cache>,
}

impl FabricMetaClient {
    /// Reusing the provided `reqwest::Client`.
    pub fn new(http: Client) -> Self {
        Self {
            http,
            cache: Default::default(),
        }
    }

    async fn get_meta<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let path = path.strip_prefix("/").unwrap_or(path);

        let url = META_API.parse::<Url>().unwrap().join(path)?;

        let res = self.http.get(url).send().await?.json().await?;

        Ok(res)
    }

    /// Full database, includes all the data. **Warning:** large JSON.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn versions(&self) -> anyhow::Result<Ref<'_, (), FabricMeta>> {
        if let Some(cached) = self.cache.versions.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions").await?;

        let read = self.cache.versions.entry(()).insert(value).downgrade();

        Ok(read)
    }

    /// Lists all of the supported game versions.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn game_versions(&self) -> anyhow::Result<Ref<'_, (), Vec<Game>>> {
        if let Some(cached) = self.cache.game_versions.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions/game").await?;

        let read = self.cache.game_versions.entry(()).insert(value).downgrade();

        Ok(read)
    }

    /// Lists all of the compatible game versions for yarn.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn game_versions_yarn(&self) -> anyhow::Result<Ref<'_, (), Vec<Game>>> {
        if let Some(cached) = self.cache.game_versions_yarn.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions/game/yarn").await?;

        let read = self
            .cache
            .game_versions_yarn
            .entry(())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Lists all of the compatible game versions for intermediary.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn game_versions_intermediary(&self) -> anyhow::Result<Ref<'_, (), Vec<Game>>> {
        if let Some(cached) = self.cache.game_versions_intermediary.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions/game/intermediary").await?;

        let read = self
            .cache
            .game_versions_intermediary
            .entry(())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Lists all of the intermediary versions, stable is based of the Minecraft version.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn intermediary_versions(&self) -> anyhow::Result<Ref<'_, (), Vec<Intermediary>>> {
        if let Some(cached) = self.cache.intermediary_versions.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions/intermediary").await?;

        let read = self
            .cache
            .intermediary_versions
            .entry(())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Lists all of the intermediary for the provided game version, there will only ever be 1.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn game_intermediary_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<Ref<'_, String, [Intermediary; 1]>> {
        if let Some(cached) = self.cache.game_intermediary_versions.get(game) {
            return Ok(cached);
        }

        let path = format!("/v2/versions/intermediary/{game}");

        let value = self.get_meta(&path).await?;

        let read = self
            .cache
            .game_intermediary_versions
            .entry(game.into())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Lists all of the yarn versions, stable is based on the Minecraft version.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn yarn_versions(&self) -> anyhow::Result<Ref<'_, (), Vec<Mapping>>> {
        if let Some(cached) = self.cache.yarn_versions.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions/yarn").await?;

        let read = self.cache.yarn_versions.entry(()).insert(value).downgrade();

        Ok(read)
    }

    /// Lists all of the yarn versions for the provided game version.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn game_yarn_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<Ref<'_, String, Vec<Mapping>>> {
        if let Some(cached) = self.cache.game_yarn_versions.get(game) {
            return Ok(cached);
        }

        let path = format!("/v2/versions/yarn/{game}");

        let value = self.get_meta(&path).await?;

        let read = self
            .cache
            .game_yarn_versions
            .entry(game.into())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Lists all of the loader versions.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn loader_versions(&self) -> anyhow::Result<Ref<'_, (), Vec<Loader>>> {
        if let Some(cached) = self.cache.loader_versions.get(&()) {
            return Ok(cached);
        }

        let value = self.get_meta("/v2/versions/loader").await?;

        let read = self
            .cache
            .loader_versions
            .entry(())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// This returns a list of all the compatible loader versions for a given version of the game,
    /// along with the best version of intermediary to use for that version.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn game_loader_versions(
        &self,
        game: &str,
    ) -> anyhow::Result<Ref<'_, String, Vec<LoaderWithIntermediary>>> {
        if let Some(cached) = self.cache.game_loader_versions.get(game) {
            return Ok(cached);
        }

        let path = format!("/v2/versions/loader/{game}");

        let value = self.get_meta(&path).await?;

        let read = self
            .cache
            .game_loader_versions
            .entry(game.into())
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Returns the JSON file that should be used in the standard Minecraft launcher.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn profile(
        &self,
        game: &str,
        loader: &str,
    ) -> anyhow::Result<Ref<'_, (String, String), Profile>> {
        if let Some(cached) = self.cache.profile.get(&(game.into(), loader.into())) {
            return Ok(cached);
        }

        let path = format!("/v2/versions/loader/{game}/{loader}/profile/json");

        let value = self.get_meta(&path).await?;

        let read = self
            .cache
            .profile
            .entry((game.into(), loader.into()))
            .insert(value)
            .downgrade();

        Ok(read)
    }

    /// Returns the JSON file in format of the launcher JSON, but with the server's main class.
    ///
    /// Note that repeated calls when holding the returned reference might result in a deadlock.
    pub async fn server(
        &self,
        game: &str,
        loader: &str,
    ) -> anyhow::Result<Ref<'_, (String, String), Server>> {
        if let Some(cached) = self.cache.server.get(&(game.into(), loader.into())) {
            return Ok(cached);
        }

        let path = format!("/v2/versions/loader/{game}/{loader}/server/json");

        let value = self.get_meta(&path).await?;

        let read = self
            .cache
            .server
            .entry((game.into(), loader.into()))
            .insert(value)
            .downgrade();

        Ok(read)
    }
}

#[derive(Default)]
struct Cache {
    versions: DashMap<(), FabricMeta>,

    game_versions: DashMap<(), Vec<Game>>,

    game_versions_yarn: DashMap<(), Vec<Game>>,

    game_versions_intermediary: DashMap<(), Vec<Game>>,

    intermediary_versions: DashMap<(), Vec<Intermediary>>,

    game_intermediary_versions: DashMap<String, [Intermediary; 1]>,

    yarn_versions: DashMap<(), Vec<Mapping>>,

    game_yarn_versions: DashMap<String, Vec<Mapping>>,

    loader_versions: DashMap<(), Vec<Loader>>,

    game_loader_versions: DashMap<String, Vec<LoaderWithIntermediary>>,

    profile: DashMap<(String, String), Profile>,

    server: DashMap<(String, String), Server>,
}
