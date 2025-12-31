use iced::Element;
use iced::widget::{row, text};
use iced::window::Id;
use crate::ui::main_frontend::{Message, Window};

pub struct KeyPressWindow;

impl Window for KeyPressWindow {
    fn view(&self, _window_id: Id) -> Element<'_, Message> {
        row![text("This is a new window!")].into()
    }
}
