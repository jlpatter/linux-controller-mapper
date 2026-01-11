use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use directories::ProjectDirs;
use enigo::Key;
use gilrs::{Axis, Button, Gamepad, GamepadId, Gilrs};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

fn get_config_path() -> Result<PathBuf> {
    let pd = ProjectDirs::from("com", "Patterson", "Linux Controller Mapper").ok_or(anyhow!("Failed to determine HOME directory on your OS"))?;
    let mut config_path_buf = pd.config_dir().to_path_buf();
    config_path_buf.push(PathBuf::from("config.json"));
    Ok(config_path_buf)
}

pub struct ActiveProfileConfig {
    gamepad_config_map: HashMap<GamepadId, GamepadConfig>,
}

impl ActiveProfileConfig {
    // TODO: Instead of generating a config by default, we should load from file first!
    pub fn new() -> Result<Self> {
        // TODO: We should capture this context instead of creating separate new ones here and in controller_handler!
        let gilrs = Gilrs::new().map_err(|e| {
            anyhow!(format!("Unable to load Gamepad Input Library (Gilrs): {}", e.to_string()))
        })?;

        let mut gamepad_config_map: HashMap<GamepadId, GamepadConfig> = HashMap::new();
        for (gp_id, gp) in gilrs.gamepads() {
            gamepad_config_map.insert(gp_id, GamepadConfig::new(gp));
        }

        Ok(Self {
            gamepad_config_map
        })
    }

    pub fn get_key(&self, gp_id: &GamepadId, btn: &Button) -> Option<&Key> {
        self.gamepad_config_map.get(gp_id)?.get_key(btn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProfileConfig {
    gamepad_configs: Vec<GamepadConfig>,
}

impl ProfileConfig {
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

#[derive(Serialize, Deserialize)]
struct GamepadConfig {
    controller_name: String,
    controller_uuid: Uuid,
    button_map: HashMap<Button, Key>,
    axis_map: HashMap<Axis, String>,
}

impl GamepadConfig {
    pub fn new(gamepad: Gamepad) -> Self {
        // TODO: Remove this after testing!
        let mut button_map: HashMap<Button, Key> = HashMap::new();
        button_map.insert(Button::South, Key::Unicode('A'));
        button_map.insert(Button::West, Key::Unicode('X'));
        button_map.insert(Button::North, Key::Unicode('Y'));
        button_map.insert(Button::East, Key::Unicode('B'));

        Self {
            controller_name: String::from(gamepad.name()),
            controller_uuid: Uuid::from_bytes(gamepad.uuid()),
            // TODO: Remove this after testing!
            button_map,
            axis_map: HashMap::new(),
        }
    }

    pub fn get_key(&self, btn: &Button) -> Option<&Key> {
        self.button_map.get(btn)
    }
}