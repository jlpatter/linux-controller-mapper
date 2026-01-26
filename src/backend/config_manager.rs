use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use directories::{BaseDirs, ProjectDirs};
use enigo::Key;
use gilrs::{Axis, Button, GamepadId, Gilrs};
use iced::keyboard;
use iced::keyboard::Key::{Character};
use iced::keyboard::key::Named;
use rfd::FileDialog;
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

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            gamepad_configs: vec![GamepadConfig::default()],
        }
    }
}

impl ProfileConfig {
    pub fn load() -> Result<Option<Self>> {
        let file_path_opt = FileDialog::new()
            .add_filter("profile", &["lcm", "json"])
            .set_directory(BaseDirs::new().ok_or(anyhow!("ERROR: Home directory not found!"))?.home_dir())
            .pick_file();

        if let Some(file_path) = file_path_opt {
            let data_string = fs::read_to_string(file_path)?;
            return Ok(Some(serde_json::from_str(&*data_string)?));
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

    pub fn get_first_gamepad_config(&self) -> &GamepadConfig {
        // TODO: This function is temporary until proper multi-controller support is implemented!
        &self.gamepad_configs[0]
    }

    pub fn get_gamepad_config(&self, index: usize) -> Option<GamepadConfig> {
        if let Some(gc) = self.gamepad_configs.get(index) {
            return Some(gc.clone());
        }
        None
    }

    pub fn get_gamepad_config_len(&self) -> usize {
        self.gamepad_configs.len()
    }

    pub fn get_gamepad_config_map(&self, gilrs: &Gilrs) -> HashMap<GamepadId, GamepadConfig> {
        // The reason we can't store this HashMap directly is that GamepadId is not static between runs.
        let mut gamepad_config_map: HashMap<GamepadId, GamepadConfig> = HashMap::new();

        // TODO: Need to come up with a better way to assign gamepads to configs!
        let mut connected_gamepad_iter = gilrs.gamepads();
        for gc in &self.gamepad_configs {
            if let Some((gamepad_id, _)) = connected_gamepad_iter.next() {
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GamepadConfig {
    button_map: HashMap<Button, Key>,
    axis_map: HashMap<Axis, String>,
}

impl GamepadConfig {
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
}
