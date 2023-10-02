use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

pub fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return home::home_dir();
    }
    home::home_dir().map(|mut h| {
        if h == Path::new("/") {
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

pub fn link<P: AsRef<Path>>(from_path: P, to_path: P) -> Result<()> {
    let from_path = from_path.as_ref();
    let to_path = to_path.as_ref();

    if !from_path.exists() {
        bail!(
            "Cannnot find '{:?}'!",
            from_path.file_name().unwrap().to_str().unwrap()
        );
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::{symlink_dir, symlink_file};

        if from_path.is_dir() {
            symlink_dir(from_path, to_path)?;
        } else if from_path.is_file() {
            symlink_file(from_path, to_path)?;
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        symlink(from_path, to_path)?;
    }

    Ok(())
}
