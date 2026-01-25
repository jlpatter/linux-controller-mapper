use std::sync::{Arc, Mutex};
use gilrs::Gilrs;
use iced::Element;
use iced::widget::{row, text};
use crate::ui::application::Message;
use crate::ui::window::base::{Window, WindowType};

pub struct KeyPressWindow;

impl Window for KeyPressWindow {
    fn window_type(&self) -> WindowType {
        WindowType::KeyPress
    }

    fn view(&self, _gilrs: Arc<Mutex<Gilrs>>) -> Element<'_, Message> {
        row![text("Please press a key to assign it.")].into()
    }
}
