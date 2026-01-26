use crate::ui::application::Message;
use crate::ui::window::base::{Window, WindowType};
use iced::Element;
use iced::widget::text;

pub struct ErrorWindow {
    error: String,
}

impl ErrorWindow {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl Window for ErrorWindow {
    fn window_type(&self) -> WindowType {
        WindowType::Error
    }

    fn view(&self) -> Element<'_, Message> {
        text(format!("The following error has occurred: {}", self.error)).into()
    }
}
