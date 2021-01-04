#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate derive_builder;
extern crate flate2;
extern crate ion_shell;
extern crate reqwest;
extern crate semver;
extern crate serde;
extern crate structopt;
extern crate tar;
extern crate toml;
extern crate url;

use crate::commands::new::NewOpt;
use crate::commands::status::status;
use crate::manifest::Manifest;
use crate::utils::find_up::find_upwards;
use crate::workspace::Workspace;
use anyhow::Result;
use std::io::Read;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use toml::from_str;

mod assets;
mod bin;
mod commands;
mod manifest;
mod package_reference;
mod scripts;
mod source;
mod utils;
mod var;
mod workspace;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    Parse { manifest: Option<PathBuf> },
    ParseWorkspace { workspace: Option<PathBuf> },
    Status { manifest: Option<PathBuf> },
    New(NewOpt),
}

fn parse_manifest(path: &PathBuf) -> Result<Manifest> {
    if path.exists() == false {
        Err(anyhow!("Package manifest doesn't exist"))
    } else {
        let mut manifest_file = std::fs::File::open(path)?;
        let mut manifest_body = String::new();
        manifest_file.read_to_string(&mut manifest_body)?;
        let manifest = from_str::<Manifest>(&manifest_body)?;
        Ok(manifest)
    }
}

fn parse_workspace(path: &PathBuf) -> Result<Workspace> {
    if path.exists() == false {
        Err(anyhow!("Workspace definition doesn't exist"))
    } else {
        let mut workspace_file = std::fs::File::open(path)?;
        let mut workspace_body = String::new();
        workspace_file.read_to_string(&mut workspace_body)?;
        let workspace = from_str::<Workspace>(&workspace_body)?;
        Ok(workspace)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Check the ppk requirements for the system

    let opt = Opt::from_args();
    match opt {
        Opt::Parse { manifest } => {
            let manifest = if let Some(manifest) = manifest {
                manifest
            } else {
                std::env::current_dir()?
            };
            let manifest = find_upwards(&manifest, "manifest.toml", |path| {
                parse_manifest(path).is_ok()
            })?;
            let manifest = parse_manifest(&manifest)?;
            println!("{:#?}", manifest);
            Ok(())
        }
        Opt::ParseWorkspace { workspace } => {
            let workspace = if let Some(workspace) = workspace {
                workspace
            } else {
                std::env::current_dir()?
            };
            let workspace = find_upwards(&workspace, "workspace.toml", |path| {
                println!("Checking: {:?}", path);
                parse_workspace(path).is_ok()
            })?;
            let workspace = parse_workspace(&workspace)?;
            println!("{:#?}", workspace);
            Ok(())
        }
        Opt::Status { manifest } => {
            let manifest = if let Some(manifest) = manifest {
                manifest
            } else {
                std::env::current_dir()?
            };
            let manifest = find_upwards(&manifest, "manifest.toml", |path| {
                parse_manifest(path).is_ok()
            })?;
            let manifest = parse_manifest(&manifest)?;
            status(&manifest);
            Ok(())
        }
        _ => Ok(()),
    }
}
