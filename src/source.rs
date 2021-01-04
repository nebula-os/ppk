use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum SourceReference {
    Alpine(AlpineReference),
    Git(GitReference),
    Download(DownloadReference),
}

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AlpineReference {
    pub repository: String,
    pub pkg: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct GitReference {
    pub git: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
}

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DownloadReference {
    url: String,
}
