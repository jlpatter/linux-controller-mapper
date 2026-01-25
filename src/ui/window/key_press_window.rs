use std::collections::HashMap;
use gilrs::GamepadId;
use iced::Element;
use iced::widget::{row, text};
use crate::backend::config_manager::GamepadConfig;
use crate::ui::application::{Message};
use crate::ui::window::base::{Window, WindowType};

pub struct KeyPressWindow;

impl Window for KeyPressWindow {
    fn window_type(&self) -> WindowType {
        WindowType::KeyPress
    }

    fn view(&self, _active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message> {
        row![text("Please press a key to assign it.")].into()
    }
}
