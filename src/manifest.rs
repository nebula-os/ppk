use crate::package_reference::PackageReference;
use crate::source::SourceReference;
use crate::var::Var;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    name: String,
    version: Version,
    src: Option<HashMap<String, SourceReference>>,
    dependencies: Option<HashMap<String, PackageReference>>,
    build_dependencies: Option<HashMap<String, PackageReference>>,
    vars: Option<HashMap<String, Var>>,
}
