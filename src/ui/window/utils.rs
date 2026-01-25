use iced::widget::{text, Text};

pub fn header<'a>(content: &'a str) -> Text<'a> {
    text(content).size(20)
}
