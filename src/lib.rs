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

pub const META_API: &str = "https://meta.fabricmc.net/";

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

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Game {
    /// The version of the game.
    ///
    /// Minecraft's version number may not be a valid semver.
    pub version: String,

    pub stable: bool,
}

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

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Intermediary {
    #[serde_as(as = "DisplayFromStr")]
    pub maven: MavenCoord,

    pub version: String,

    pub stable: bool,
}

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

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LoaderWithIntermediary {
    pub loader: Loader,

    pub intermediary: Intermediary,

    #[serde(rename = "launcherMeta", skip_serializing)]
    pub _launcher_meta: Option<IgnoredAny>,
}

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
