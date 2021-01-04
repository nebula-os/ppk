#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate derive_builder;
extern crate semver;
extern crate serde;
extern crate structopt;
extern crate toml;
extern crate url;

use crate::manifest::Manifest;
use anyhow::Result;
use std::io::Read;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use toml::from_str;

mod commands;
mod manifest;
mod package_reference;
mod source;
mod var;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    Parse { manifest: PathBuf },
    Status { manifest: PathBuf },
}

fn parse_manifest(path: &PathBuf) -> Result<Manifest> {
    if path.exists() == false {
        Err(anyhow!("Manifest doesn't exist"))
    } else {
        let mut manifest_file = std::fs::File::open(path)?;
        let mut manifest_body = String::new();
        manifest_file.read_to_string(&mut manifest_body)?;

        let manifest = from_str::<Manifest>(&manifest_body)?;
        Ok(manifest)
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Parse { manifest } => {
            let manifest = parse_manifest(&manifest)?;
            println!("{:#?}", manifest);
            Ok(())
        }
        Opt::Status { manifest } => {
            let manifest = parse_manifest(&manifest)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
