use crate::backend::config_manager::GamepadConfig;
use crate::backend::joysticks::Joystick;
use crate::backend::key_utils::MouseButtonOrKey;
use crate::ui::application::Message;
use crate::ui::window::mouse_button_wrapper::MouseButtonWrapper;
use enigo::Button as MouseButton;
use gilrs::Button;
use iced::widget::{Row, Text, button, container, pick_list, row, text};
use iced::{Color, Length};

pub fn header<'a>(content: &'a str) -> Text<'a> {
    text(content).size(20)
}

pub fn joystick_row<'c>(label: &'c str, joystick: Joystick, is_in_use: bool) -> Row<'c, Message> {
    let msg = if is_in_use {
        "controlling the mouse. ✔️"
    } else {
        "NOT controlling the mouse. ❌️"
    };

    row![
        text(label).color(Color::from_rgb8(255, 0, 0)),
        text(" is currently "),
        text(msg).color(Color::from_rgb8(0, 0, 255)),
        container(button("Toggle").on_press(Message::ToggleAxisSelection(joystick)))
            .padding([0, 10]),
    ]
    .width(Length::Fill)
}

fn get_str_from_config(gc: &GamepadConfig, gilrs_btn: &Button) -> String {
    // TODO: Move gc.get_key(...) out of this function!
    if let Some(key) = gc.get_key(gilrs_btn) {
        return key.to_string();
    }
    "None".to_string()
}

pub fn button_mapper_row<'b>(label: &'b str, btn: Button, gc: &GamepadConfig) -> Row<'b, Message> {
    let mouse_buttons = [
        MouseButtonWrapper(MouseButton::Left),
        MouseButtonWrapper(MouseButton::Middle),
        MouseButtonWrapper(MouseButton::Right),
        MouseButtonWrapper(MouseButton::Back),
        MouseButtonWrapper(MouseButton::Forward),
        MouseButtonWrapper(MouseButton::ScrollUp),
        MouseButtonWrapper(MouseButton::ScrollDown),
        MouseButtonWrapper(MouseButton::ScrollLeft),
        MouseButtonWrapper(MouseButton::ScrollRight),
    ];

    let mut selected_mouse_button: Option<MouseButtonWrapper> = None;
    if let Some(mb_key) = gc.get_key(&btn) {
        if let MouseButtonOrKey::MouseButton(mb) = mb_key {
            selected_mouse_button = Some(MouseButtonWrapper(*mb));
        }
    }

    row![
        text(label).color(Color::from_rgb8(255, 0, 0)),
        text(" is currently assigned to: ".to_string()),
        text(get_str_from_config(gc, &btn)).color(Color::from_rgb8(0, 0, 255)),
        container(button("Assign Key").on_press(Message::OpenKeySetWindow(btn))).padding([0, 10]),
        container(button("Unassign").on_press(Message::UnsetButton(btn))).padding([0, 10]),
        pick_list(mouse_buttons, selected_mouse_button, move |mbw| {
            Message::MouseButtonSet(btn, mbw.0)
        })
        .placeholder("Select a mouse button..."),
    ]
    .width(Length::Fill)
}
