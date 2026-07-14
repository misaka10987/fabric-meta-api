#[cfg(feature = "client")]
mod client;
mod prelude;

#[allow(unused_imports)]
pub use prelude::*;

use creeper_maven_coord::MavenCoord;
use mc_launchermeta::{VersionKind, version::Arguments};
use serde::{Deserialize, Serialize, de::IgnoredAny};
use serde_with::{DisplayFromStr, serde_as};
use url::Url;

/// Officially hosted at [https://meta.fabricmc.net/](https://meta.fabricmc.net/).
pub const META_API: &str = "https://meta.fabricmc.net/";

/// See [`/v2/versions`](https://github.com/FabricMC/fabric-meta#v2versions).
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FabricMeta {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub game: Vec<Game>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<Mapping>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intermediary: Vec<Intermediary>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loader: Vec<Loader>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub installer: Vec<Installer>,
}

/// See [`/v2/versions/game`](https://github.com/FabricMC/fabric-meta#v2versionsgame).
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Game {
    /// The version of the game.
    ///
    /// Minecraft's version number may not be a valid semver.
    pub version: String,

    pub stable: bool,
}

/// See [`/v2/versions/yarn`](https://github.com/FabricMC/fabric-meta#v2versionsyarn).
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Mapping {
    pub game_version: String,

    pub separator: String,

    pub build: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub maven: MavenCoord,

    pub version: String,

    pub stable: bool,
}

/// See [`/v2/versions/intermediary`](https://github.com/FabricMC/fabric-meta#v2versionsintermediary).
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Intermediary {
    #[serde_as(as = "DisplayFromStr")]
    pub maven: MavenCoord,

    pub version: String,

    pub stable: bool,
}

/// See [`/v2/versions/loader`](https://github.com/FabricMC/fabric-meta#v2versionsloader).
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Loader {
    pub separator: String,

    pub build: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub maven: MavenCoord,

    pub version: String,

    pub stable: bool,
}

/// See [`/v2/versions`](https://github.com/FabricMC/fabric-meta#v2versions).
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Installer {
    pub url: Url,

    #[serde_as(as = "DisplayFromStr")]
    pub maven: MavenCoord,

    pub version: String,

    pub stable: bool,
}

/// See [`/v2/versions/loader/:game_version`](https://github.com/FabricMC/fabric-meta#v2versionsloadergame_version).
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LoaderWithIntermediary {
    pub loader: Loader,

    pub intermediary: Intermediary,

    #[serde(rename = "launcherMeta", skip_serializing)]
    pub _launcher_meta: Option<IgnoredAny>,
}

/// See [`/v2/versions/loader/:game_version/:loader_version/profile/json`](https://github.com/FabricMC/fabric-meta#v2versionsloadergame_versionloader_versionprofilejson).
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Profile {
    pub id: String,

    pub inherits_from: String,

    pub release_time: String,

    pub time: String,

    #[serde(rename = "type")]
    pub kind: VersionKind,

    pub main_class: String,

    pub arguments: Arguments,

    pub libraries: Vec<Library>,
}

/// See [`/v2/versions/loader/:game_version/:loader_version/profile/json`](https://github.com/FabricMC/fabric-meta#v2versionsloadergame_versionloader_versionprofilejson).
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Library {
    #[serde_as(as = "DisplayFromStr")]
    pub name: MavenCoord,

    pub url: Url,

    pub md5: Option<String>,

    pub sha1: Option<String>,

    pub sha256: Option<String>,

    pub sha512: Option<String>,

    pub size: Option<u64>,
}
