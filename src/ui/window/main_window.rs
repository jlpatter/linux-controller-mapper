use std::collections::HashMap;
use enigo::Key::Unicode;
use gilrs::{Button, GamepadId};
use iced::{Color, Element, Length};
use iced::widget::{button, container, row, scrollable, text, Row};
use iced::window::Id;
use crate::backend::config_manager::GamepadConfig;
use crate::ui::application::{Message};
use crate::ui::window::utils::header;
use crate::ui::window::base::Window;

pub struct MainWindow;

impl MainWindow {
    fn button_mapper_row<'b>(label: &'b str, btn: Button, gc: &GamepadConfig) -> Row<'b, Message> {
        row![
            text(label).color(Color::from_rgb8(255, 0, 0)),
            text(" is currently assigned to: ".to_string()),
            text(Self::get_str_from_config(gc, &btn)).color(Color::from_rgb8(0, 0, 255)),
            container(button("Set").on_press(Message::OpenKeySetWindow(btn))).padding([0, 10]),
            container(button("Unset").on_press(Message::UnsetButton(btn))).padding([0, 10]),
        ].width(Length::Fill)
    }

    fn get_str_from_config(gc: &GamepadConfig, gilrs_btn: &Button) -> String {
        if let Some(key) = gc.get_key(gilrs_btn) {
            if let Unicode(u) = key  {
                return u.to_string().to_uppercase();
            } else {
                // TODO: Technically I guess we should move away from using the debug form?
                return format!("{:?}", key);
            }
        }
        "None".to_string()
    }
}

impl Window for MainWindow {
    fn view(&self, _window_id: Id, active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message> {
        // TODO: Add a dropdown to support multiple gamepads!
        let single_active_gamepad_config = active_gamepad_config_map.values().next().unwrap();

        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);

        scrollable(iced::widget::column![
            header("Menu Pad"),
            MainWindow::button_mapper_row("Start", Button::Start, single_active_gamepad_config),
            MainWindow::button_mapper_row("Select", Button::Select, single_active_gamepad_config),
            MainWindow::button_mapper_row("Mode", Button::Mode, single_active_gamepad_config),

            header("Action Pad"),
            MainWindow::button_mapper_row("North", Button::North, single_active_gamepad_config),
            MainWindow::button_mapper_row("West", Button::West, single_active_gamepad_config),
            MainWindow::button_mapper_row("East", Button::East, single_active_gamepad_config),
            MainWindow::button_mapper_row("South", Button::South, single_active_gamepad_config),

            header("Sticks"),
            MainWindow::button_mapper_row("Left Stick Press", Button::LeftThumb, single_active_gamepad_config),
            MainWindow::button_mapper_row("Right Stick Press", Button::RightThumb, single_active_gamepad_config),

            header("Triggers"),
            MainWindow::button_mapper_row("Left Bumper", Button::LeftTrigger, single_active_gamepad_config),
            MainWindow::button_mapper_row("Left Trigger", Button::LeftTrigger2, single_active_gamepad_config),
            MainWindow::button_mapper_row("Right Bumper", Button::RightTrigger, single_active_gamepad_config),
            MainWindow::button_mapper_row("Right Trigger", Button::RightTrigger2, single_active_gamepad_config),

            header("D-Pad"),
            MainWindow::button_mapper_row("Up", Button::DPadUp, single_active_gamepad_config),
            MainWindow::button_mapper_row("Left", Button::DPadLeft, single_active_gamepad_config),
            MainWindow::button_mapper_row("Right", Button::DPadRight, single_active_gamepad_config),
            MainWindow::button_mapper_row("Down", Button::DPadDown, single_active_gamepad_config),

            header("Misc."),
            MainWindow::button_mapper_row("C Button", Button::C, single_active_gamepad_config),
            MainWindow::button_mapper_row("Z Button", Button::Z, single_active_gamepad_config),

            row![container(activate).padding([0, 10]), deactivate],
        ].spacing(5)).into()
    }
}
