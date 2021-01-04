use crate::assets::SCRIPT_SETUP_ALPINE;
use anyhow::Result;
use ion_shell::Shell;
use std::io::BufReader;

pub fn setup_alpine(mirror: &str, version: &str, chroot_dir: &str, branch: &str) -> Result<()> {
    let mut shell = Shell::default();
    let script = String::from(SCRIPT_SETUP_ALPINE);
    let script = script.replace("<mirror>", mirror);
    let script = script.replace("<version>", version);
    let script = script.replace("<chroot_dir>", chroot_dir);
    let script = script.replace("<branch>", branch);
    let buf = BufReader::new(script.as_bytes());
    shell.execute_command(buf)?;

    Ok(())
}
