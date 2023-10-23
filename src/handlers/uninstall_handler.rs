use std::path::Path;

use anyhow::{anyhow, Result};
use reqwest::Client;
use tokio::fs;
use tracing::{info, warn};

use crate::{config::Config, helpers::{self, directories, version::types::VersionType}};

pub async fn start(version: &str, config: Config) -> Result<()> {
    let client = Client::new();
    let version = helpers::version::parse_version_type(&client, version).await?;

    if helpers::version::is_version_used(&version.tag_name, &config).await {
        warn!("Switch to a different version before proceeding");
        return Ok(());
    }

    let downloads_dir = match directories::get_downloads_directory(&config).await {
        Ok(value) => value,
        Err(error) => return Err(anyhow!(error)),
    };

    let path = downloads_dir.join(&version.tag_name);

    fs::remove_dir_all(path).await?;
    info!("Successfully uninstalled version: {}", version.tag_name);
    Ok(())
}
