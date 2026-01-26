use crate::ui::application::Message;
use crate::ui::window::base::{Window, WindowType};
use iced::Element;
use iced::widget::{row, text};

pub struct KeyPressWindow;

impl Window for KeyPressWindow {
    fn window_type(&self) -> WindowType {
        WindowType::KeyPress
    }

    fn view(&self) -> Element<'_, Message> {
        row![text("Please press a key to assign it.")].into()
    }
}
