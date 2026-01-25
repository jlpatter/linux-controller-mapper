use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use directories::ProjectDirs;
use enigo::Key;
use gilrs::{Axis, Button, Gamepad, Gilrs};
use iced::keyboard;
use iced::keyboard::Key::{Character};
use iced::keyboard::key::Named;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

fn get_config_path() -> Result<PathBuf> {
    let pd = ProjectDirs::from("com", "Patterson", "Linux Controller Mapper").ok_or(anyhow!("Failed to determine HOME directory on your OS"))?;
    let mut config_path_buf = pd.config_dir().to_path_buf();
    config_path_buf.push(PathBuf::from("config.json"));
    Ok(config_path_buf)
}

#[derive(Serialize, Deserialize)]
pub struct ProfileConfig {
    gamepad_configs: Vec<GamepadConfig>,
}

impl ProfileConfig {
    pub fn load(gilrs: Arc<Mutex<Gilrs>>) -> Result<Self> {
        let config_path_buf = get_config_path()?;
        let config_path = config_path_buf.as_path();
        let mut profile_config: ProfileConfig;

        if !config_path.exists() {
            profile_config = Self {
                gamepad_configs: Vec::new(),
            }
        } else {
            let data_string = fs::read_to_string(config_path)?;
            profile_config = serde_json::from_str(&*data_string)?;
        }

        // Add any missing gamepads as empty configs
        for (_, gamepad) in gilrs.lock().unwrap().gamepads() {
            let gc_search_result = profile_config.gamepad_configs.iter().find(|gc| {
                Uuid::from_bytes(gamepad.uuid()) == *gc.uuid()
            });
            if gc_search_result.is_none() {
                profile_config.gamepad_configs.push(GamepadConfig::new(&gamepad));
            }
        }

        Ok(profile_config)
    }

    pub fn insert_key_to_all(&mut self, btn: Button, key: keyboard::Key) {
        // TODO: This function is temporary until proper multi-controller support is implemented!
        for gc in &mut self.gamepad_configs {
            gc.insert_key(btn.clone(), key.clone());
        }
    }

    pub fn unset_key_to_all(&mut self, btn: Button) {
        // TODO: This function is temporary until proper multi-controller support is implemented!
        for gc in &mut self.gamepad_configs {
            gc.remove_key(btn);
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path_buf = get_config_path()?;
        let config_path = config_path_buf.as_path();
        if !config_path.exists() {
            let prefix = config_path.parent().ok_or(anyhow!("Config path prefix not defined. This should never happen if the directories library is working."))?;
            if !prefix.exists() {
                create_dir_all(prefix)?;
            }
        }
        fs::write(config_path, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }

    pub fn gamepad_configs(&self) -> &Vec<GamepadConfig> {
        &self.gamepad_configs
    }
}

fn get_enigo_key_from_iced_key(key: keyboard::Key) -> Option<Key> {
    if let Character(c) = key {
        Some(Key::Unicode(c.chars().next()?))
    } else if let keyboard::Key::Named(named) = key {
        let named_chars_map: HashMap<Named, Key> = HashMap::from([
            (Named::Control, Key::Control),
            (Named::Alt, Key::Alt),
            (Named::Shift, Key::Shift),
            (Named::Tab, Key::Tab),
            (Named::Escape, Key::Escape),
            (Named::Meta, Key::Meta),
            (Named::Backspace, Key::Backspace),
        ]);

        Some(*named_chars_map.get(&named)?)
    } else {
        None
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GamepadConfig {
    controller_name: String,
    controller_uuid: Uuid,
    button_map: HashMap<Button, Key>,
    axis_map: HashMap<Axis, String>,
}

impl GamepadConfig {
    pub fn new(gamepad: &Gamepad) -> Self {
        Self {
            controller_name: gamepad.name().to_string(),
            controller_uuid: Uuid::from_bytes(gamepad.uuid()),
            button_map: HashMap::new(),
            axis_map: HashMap::new(),
        }
    }

    pub fn insert_key(&mut self, btn: Button, key: keyboard::Key) {
        if let Some(k) = get_enigo_key_from_iced_key(key) {
            self.button_map.insert(btn, k);
        }
    }

    pub fn remove_key(&mut self, btn: Button) {
        self.button_map.remove(&btn);
    }

    pub fn get_key(&self, btn: &Button) -> Option<&Key> {
        self.button_map.get(btn)
    }

    pub fn uuid(&self) -> &Uuid {
        &self.controller_uuid
    }
}
