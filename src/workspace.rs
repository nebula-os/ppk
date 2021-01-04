use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Workspace {
    name: String,
    #[serde(default)]
    alpine: Alpine,
}

fn default_alpine_mirror() -> String {
    String::from("http://dl-cdn.alpinelinux.org/alpine/")
}
fn default_apk_tools_static_version() -> String {
    String::from("2.10.5-r1")
}
fn default_branch() -> String {
    String::from("v3.3")
}

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Alpine {
    #[serde(default = "default_alpine_mirror")]
    mirror: String,
    #[serde(default = "default_apk_tools_static_version")]
    apk_tools_static_version: String,
    #[serde(default = "default_branch")]
    branch: String,
}

impl Default for Alpine {
    fn default() -> Self {
        Alpine {
            mirror: default_alpine_mirror(),
            apk_tools_static_version: default_apk_tools_static_version(),
            branch: default_branch(),
        }
    }
}
