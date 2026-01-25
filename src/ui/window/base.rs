use std::sync::{Arc, Mutex};
use gilrs::Gilrs;
use iced::Element;
use crate::ui::application::Message;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum WindowType {
    Main,
    KeyPress,
    Error
}

pub trait Window {
    fn window_type(&self) -> WindowType;
    fn view(&self, gilrs: Arc<Mutex<Gilrs>>) -> Element<'_, Message>;
}
