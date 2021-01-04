use anyhow::Result;
use std::path::PathBuf;

pub fn find_upwards<V>(path: &PathBuf, file: &str, validator: V) -> Result<PathBuf>
where
    V: Fn(&PathBuf) -> bool,
{
    if path.exists() == false {
        Err(anyhow!("File \"{}\" wasn't found", file))
    } else {
        if path.is_file() {
            // It is a file path
            let valid = validator(path) && path.file_name().unwrap() == file;
            if valid {
                Ok(PathBuf::from(path))
            } else {
                Err(anyhow!("File \"{}\" wasn't found", file))
            }
        } else {
            // It is a directory path
            let file_path = path.join(file);
            let valid = file_path.exists() && validator(&file_path);
            if valid {
                Ok(PathBuf::from(file_path))
            } else {
                let parent = path.parent();
                if let Some(parent) = parent {
                    find_upwards(&parent.into(), file, validator)
                } else {
                    Err(anyhow!("File \"{}\" wasn't found", file))
                }
            }
        }
    }
}
