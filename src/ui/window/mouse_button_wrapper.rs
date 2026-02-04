use enigo::{Button as MouseButton, Button};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct MouseButtonWrapper(pub MouseButton);

impl Display for MouseButtonWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Button::Left => write!(f, "Left"),
            Button::Middle => write!(f, "Middle"),
            Button::Right => write!(f, "Right"),
            Button::Back => write!(f, "Back"),
            Button::Forward => write!(f, "Forward"),
            Button::ScrollUp => write!(f, "ScrollUp"),
            Button::ScrollDown => write!(f, "ScrollDown"),
            Button::ScrollLeft => write!(f, "ScrollLeft"),
            Button::ScrollRight => write!(f, "ScrollRight"),
        }
    }
}
