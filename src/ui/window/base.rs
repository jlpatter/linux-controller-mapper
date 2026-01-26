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
    fn view(&self) -> Element<'_, Message>;
}
