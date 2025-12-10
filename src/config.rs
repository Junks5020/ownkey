use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use directories::BaseDirs;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncProvider {
    LocalOnly,
    File,
    Http,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_sync_provider")]
    pub sync_provider: SyncProvider,
}

fn default_sync_provider() -> SyncProvider {
    SyncProvider::LocalOnly
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sync_provider: SyncProvider::LocalOnly,
        }
    }
}

fn config_dir() -> Result<PathBuf> {
    let base =
        BaseDirs::new().ok_or_else(|| anyhow!("cannot resolve home directory for config"))?;
    let dir = base.home_dir().join(".ownkey");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn load_or_init() -> Result<Config> {
    let path = config_path()?;

    if !path.exists() {
        let default_cfg = Config::default();
        let template = r#"# ownkey configuration

# sync_provider controls how vault sync works.
# Supported values:
#   "local_only" - no remote sync (default)
#   "file"       - sync to a local/remote file path
#   "http"       - sync via HTTP backend

sync_provider = "local_only"
"#;
        fs::write(&path, template)
            .with_context(|| format!("failed to write default config to {}", path.display()))?;
        return Ok(default_cfg);
    }

    let contents = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file {}", path.display()))?;
    let cfg: Config = toml::from_str(&contents)
        .with_context(|| format!("failed to parse config file {}", path.display()))?;
    Ok(cfg)
}

