use std::collections::HashMap;
use gilrs::GamepadId;
use iced::Element;
use iced::widget::{row, text};
use iced::window::Id;
use crate::backend::config_manager::GamepadConfig;
use crate::ui::main_frontend::{Message, Window};

pub struct KeyPressWindow;

impl Window for KeyPressWindow {
    fn view(&self, _window_id: Id, _active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message> {
        row![text("Please press a key to assign it.")].into()
    }
}
