use std::collections::HashMap;
use gilrs::GamepadId;
use iced::Element;
use crate::backend::config_manager::GamepadConfig;
use crate::ui::application::Message;

#[derive(PartialEq)]
pub enum WindowType {
    Main,
    KeyPress
}

pub trait Window {
    fn window_type(&self) -> WindowType;
    fn view(&self, active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message>;
}
