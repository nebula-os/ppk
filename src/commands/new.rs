use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "new",
    about = "Command for initialization of workspaces and packages."
)]
pub struct NewOpt {
    /// Workspace name
    #[structopt(long = "workspace")]
    workspace: String,

    /// Package name
    #[structopt(long = "package")]
    package: String,
}

pub fn new(opt: &NewOpt) -> Result<()> {
    Ok(())
}
