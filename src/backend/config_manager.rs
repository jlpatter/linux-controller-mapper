use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use directories::ProjectDirs;
use enigo::Key;
use gilrs::{Axis, Button};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

fn get_config_path() -> Result<PathBuf> {
    let pd = ProjectDirs::from("com", "Patterson", "Linux Controller Mapper").ok_or(anyhow!("Failed to determine HOME directory on your OS"))?;
    let mut config_path_buf = pd.config_dir().to_path_buf();
    config_path_buf.push(PathBuf::from("config.json"));
    Ok(config_path_buf)
}

#[derive(Serialize, Deserialize)]
struct LayoutConfig {
    controller_name: String,
    controller_uuid: Uuid,
    button_map: HashMap<Button, Key>,
    axis_map: HashMap<Axis, String>,
}

impl LayoutConfig {
    pub fn save(&self) -> Result<()> {
        let config_path_buf = get_config_path()?;
        let config_path = config_path_buf.as_path();
        if !config_path.exists() {
            let prefix = config_path.parent().ok_or(anyhow!("Config path prefix not defined. This should never happen if the library is working."))?;
            if !prefix.exists() {
                create_dir_all(prefix)?;
            }
        }
        fs::write(config_path, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }
}
