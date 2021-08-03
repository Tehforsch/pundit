use std::{fs::File, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use clap::crate_name;
use dirs_next::config_dir;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct Settings {
    pub pundit_folder: Option<PathBuf>,
}

impl Settings {
    pub fn from_default_location() -> Option<Self> {
        let file = config_dir()?.join(crate_name!()).join("settings.yaml");
        let handle = File::open(&file).ok()?;

        let result = serde_yaml::from_reader(handle);

        match result {
            Err(e) => {
                // TODO: proper error handling
                eprintln!(
                    "Error while reading settings from '{}': {}\nUsing default settings.",
                    file.to_string_lossy(),
                    e
                );
                None
            }
            Ok(settings) => Some(settings),
        }
    }

    pub fn expand_all_paths(&mut self) -> Result<()> {
        if let Some(ref path) = self.pundit_folder {
            self.pundit_folder = Some(expanduser(path)?);
        }
        Ok(())
    }
}

pub fn expanduser(path: &Path) -> Result<PathBuf> {
    let expanded = shellexpand::tilde(path.to_str().unwrap());
    Path::new(&*expanded)
        .canonicalize()
        .context(format!("While reading {}", &expanded))
}
