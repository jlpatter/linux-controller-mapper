use crate::backend::config_manager::GamepadConfig;
use crate::ui::application::Message;
use gilrs::Button;
use iced::widget::{Row, Text, button, container, row, text};
use iced::{Color, Length};

pub fn header<'a>(content: &'a str) -> Text<'a> {
    text(content).size(20)
}

fn get_str_from_config(gc: &GamepadConfig, gilrs_btn: &Button) -> String {
    // TODO: Move gc.get_key(...) out of this function!
    if let Some(key) = gc.get_key(gilrs_btn) {
        return key.to_string();
    }
    "None".to_string()
}

pub fn button_mapper_row<'b>(label: &'b str, btn: Button, gc: &GamepadConfig) -> Row<'b, Message> {
    row![
        text(label).color(Color::from_rgb8(255, 0, 0)),
        text(" is currently assigned to: ".to_string()),
        text(get_str_from_config(gc, &btn)).color(Color::from_rgb8(0, 0, 255)),
        container(button("Set").on_press(Message::OpenKeySetWindow(btn))).padding([0, 10]),
        container(button("Unset").on_press(Message::UnsetButton(btn))).padding([0, 10]),
    ]
    .width(Length::Fill)
}
