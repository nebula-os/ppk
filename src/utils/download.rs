use anyhow::Result;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub async fn download_file(url: &str, to: &PathBuf) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut out = File::create(to)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut out)?;
    Ok(())
}
