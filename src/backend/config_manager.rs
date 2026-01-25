use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use directories::{BaseDirs, ProjectDirs};
use enigo::Key;
use gilrs::{Axis, Button, Gamepad, GamepadId, Gilrs};
use iced::keyboard;
use iced::keyboard::Key::{Character};
use iced::keyboard::key::Named;
use rfd::FileDialog;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::utils::lock_error_handler;

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
    pub fn new(gilrs: Arc<Mutex<Gilrs>>) -> Result<Self> {
        let mut profile_config: ProfileConfig = Self {
            gamepad_configs: Vec::new(),
        };

        // Add all connected gamepads as empty configs
        for (_, gamepad) in gilrs.lock().map_err(lock_error_handler)?.gamepads() {
            profile_config.gamepad_configs.push(GamepadConfig::new(&gamepad));
        }

        Ok(profile_config)
    }

    pub fn load(gilrs: Arc<Mutex<Gilrs>>) -> Result<Option<Self>> {
        let file_path_opt = FileDialog::new()
            .add_filter("profile", &["lcm", "json"])
            .set_directory(BaseDirs::new().ok_or(anyhow!("ERROR: Home directory not found!"))?.home_dir())
            .pick_file();

        if let Some(file_path) = file_path_opt {
            let data_string = fs::read_to_string(file_path)?;
            let mut profile_config: ProfileConfig = serde_json::from_str(&*data_string)?;

            // Add any missing gamepads as empty configs
            for (_, gamepad) in gilrs.lock().map_err(lock_error_handler)?.gamepads() {
                let gc_search_result = profile_config.gamepad_configs.iter().any(|gc| {
                    Uuid::from_bytes(gamepad.uuid()) == *gc.uuid()
                });
                if !gc_search_result {
                    profile_config.gamepad_configs.push(GamepadConfig::new(&gamepad));
                }
            }

            return Ok(Some(profile_config));
        }
        Ok(None)
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
        let file_path_opt = FileDialog::new()
            .add_filter("profile", &["lcm", "json"])
            .set_directory(BaseDirs::new().ok_or(anyhow!("ERROR: Home directory not found!"))?.home_dir())
            .save_file();

        if let Some(file_path) = file_path_opt {
            fs::write(file_path, serde_json::to_string_pretty(&self)?)?;
        }

        Ok(())
    }

    pub fn get_gamepad_config_map(&self, gilrs: Arc<Mutex<Gilrs>>) -> HashMap<GamepadId, GamepadConfig> {
        // The reason we can't store this HashMap directly is that GamepadId is not static between runs.
        let mut gamepad_config_map: HashMap<GamepadId, GamepadConfig> = HashMap::new();

        for (gamepad_id, gamepad) in gilrs.lock().unwrap().gamepads() {
            let gc_search_result = self.gamepad_configs.iter().find(|gc| {
                Uuid::from_bytes(gamepad.uuid()) == *gc.uuid()
            });
            if let Some(gc) = gc_search_result {
                gamepad_config_map.insert(gamepad_id, gc.clone());
            }
        }

        gamepad_config_map
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
