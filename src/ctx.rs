use std::path::{Path, PathBuf};

use anyhow::{Context, Ok, Result};
use toml_edit::{value, Array, Document};

use crate::cli::Cli;

pub struct Ctx {
    pub dotfiles_path: PathBuf,
    pub config: Document,
    pub cli: Cli,
}

impl Ctx {
    pub fn new(cli: Cli) -> Result<Ctx> {
        let dotfiles_path = home::home_dir().unwrap().join("dotfiles");
        let config_raw = std::fs::read_to_string(Path::new(&dotfiles_path).join("vbdot.toml"))?;
        let config_doc = config_raw.parse().context("invalid doc")?;

        Ok(Ctx {
            cli,
            dotfiles_path,
            config: config_doc,
        })
    }

    pub fn add_link(&mut self, profile: &str, filename: &str, from_value: &str, to_value: &str) {
        let mut array = Array::default();
        array.push(from_value);
        array.push(to_value);

        self.config[profile]["link"][filename] = value(array);
    }

    pub fn save(&self) -> Result<()> {
        std::fs::write(
            Path::new(&self.dotfiles_path).join("vbdot.toml"),
            self.config.to_string(),
        )?;
        Ok(())
    }
}
