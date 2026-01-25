use std::collections::HashMap;
use gilrs::GamepadId;
use iced::Element;
use iced::window::Id;
use crate::backend::config_manager::GamepadConfig;
use crate::ui::application::Message;

pub trait Window {
    fn view(&self, id: Id, active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message>;
}
