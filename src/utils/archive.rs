use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

pub fn unarchive_tar_gz(file: &PathBuf, to: &PathBuf) -> Result<()> {
    // Validation
    if file.exists() == false {
        return Err(anyhow!("File doesn't exist"));
    }
    if file.extension().unwrap() != "gz" {
        return Err(anyhow!("File extension is not \".gz\""));
    }
    if file.is_dir() == false {
        return Err(anyhow!("Expected a directory, found a file"));
    }
    if file.exists() == false {
        create_dir_all(to)?;
    }

    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(to)?;

    Ok(())
}

pub fn unarchive_apk(file: &PathBuf, to: &PathBuf) -> Result<()> {
    // Validation
    if file.exists() == false {
        return Err(anyhow!("File doesn't exist"));
    }
    if file.extension().unwrap() != "apk" {
        return Err(anyhow!("File extension is not \".apk\""));
    }
    if file.is_dir() == false {
        return Err(anyhow!("Expected a directory, found a file"));
    }
    if file.exists() == false {
        create_dir_all(to)?;
    }

    let tar_gz = File::open(file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(to)?;

    Ok(())
}
