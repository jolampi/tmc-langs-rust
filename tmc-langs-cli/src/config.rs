//! Handles the CLI's configuration files and credentials.

use anyhow::{Context, Error};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

// base directory for a given plugin's settings files
fn get_tmc_dir(client_name: &str) -> Result<PathBuf, Error> {
    let config_dir = match env::var("TMC_LANGS_CONFIG_DIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => dirs::config_dir().context("Failed to find config directory")?,
    };
    Ok(config_dir.join(format!("tmc-{}", client_name)))
}

pub fn get_credentials_path(client_name: &str) -> Result<PathBuf, Error> {
    get_tmc_dir(client_name).map(|dir| dir.join("credentials.json"))
}

fn get_config_path(client_name: &str) -> Result<PathBuf, Error> {
    get_tmc_dir(client_name).map(|dir| dir.join("config.toml"))
}

fn init_config(client_name: &str, path: &Path) -> Result<HashMap<String, String>, Error> {
    let mut file = File::create(&path)
        .with_context(|| format!("Failed to create new config file at {}", path.display()))?;

    let mut config = HashMap::new();
    config.insert(
        "projects-folder".to_string(),
        get_tmc_dir(client_name)?
            .join("projects")
            .to_string_lossy()
            .into_owned(),
    );

    let toml = toml::to_string_pretty(&config).context("Failed to serialize config")?;
    file.write_all(toml.as_bytes())
        .with_context(|| format!("Failed to write default config to {}", path.display()))?;
    Ok(config)
}

pub fn load_config(client_name: &str) -> Result<HashMap<String, String>, Error> {
    let path = get_config_path(client_name)?;
    match fs::read(&path) {
        Ok(bytes) => match toml::from_slice(&bytes) {
            Ok(config) => Ok(config),
            Err(_) => {
                log::error!(
                    "Failed to deserialize config at {}, deleting",
                    path.display()
                );
                fs::remove_file(&path).with_context(|| {
                    format!("Failed to remove invalid config file at {}", path.display())
                })?;
                init_config(client_name, &path)
            }
        },
        Err(_) => init_config(client_name, &path),
    }
}

pub fn save_config(client_name: &str, config: HashMap<String, String>) -> Result<(), Error> {
    let path = get_config_path(client_name)?;
    let toml = toml::to_string_pretty(&config).context("Failed to serialize HashMap")?;
    fs::write(&path, toml.as_bytes())
        .with_context(|| format!("Failed to write TOML to {}", path.display()))?;
    Ok(())
}

pub fn reset_config(client_name: &str) -> Result<(), Error> {
    let path = get_config_path(client_name)?;
    init_config(client_name, &path)?;
    Ok(())
}