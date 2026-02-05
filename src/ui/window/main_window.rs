use crate::backend::config_manager::ProfileConfig;
use crate::backend::joysticks::Joystick;
use crate::ui::application::Message;
use crate::ui::window::base::{Window, WindowType};
use crate::ui::window::components::{button_mapper_row, header, joystick_row};
use gilrs::Button;
use iced::widget::{button, column, row, scrollable, text};
use iced::{Color, Element, Length};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct MainWindow {
    profile_config: Arc<Mutex<ProfileConfig>>,
    is_handler_running: Arc<AtomicBool>,
}

impl MainWindow {
    pub fn new(
        profile_config: Arc<Mutex<ProfileConfig>>,
        is_handler_running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            profile_config,
            is_handler_running,
        }
    }
}

impl Window for MainWindow {
    fn window_type(&self) -> WindowType {
        WindowType::Main
    }

    fn view(&self) -> Element<'_, Message> {
        let profile_config = self.profile_config.lock().unwrap();
        // TODO: Add a dropdown to support multiple gamepads!
        let single_active_gamepad_config = profile_config.get_first_gamepad_config();

        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);
        let handler_text = if self.is_handler_running.load(Ordering::Relaxed) {
            text("Controller Active!").color(Color::from_rgb8(0, 150, 0))
        } else {
            text("Controller Inactive")
        };

        let save_profile = button("Save Profile").on_press(Message::SaveProfile);
        let load_profile = button("Load Profile").on_press(Message::LoadProfile);

        column![
            scrollable(
                column![
                    header("Joystick Axes"),
                    joystick_row(
                        "Left Joystick",
                        Joystick::Left,
                        single_active_gamepad_config.use_left_stick_mouse
                    ),
                    joystick_row(
                        "Right Joystick",
                        Joystick::Right,
                        single_active_gamepad_config.use_right_stick_mouse
                    ),
                    header("Menu Pad"),
                    button_mapper_row("Start", Button::Start, single_active_gamepad_config),
                    button_mapper_row("Select", Button::Select, single_active_gamepad_config),
                    button_mapper_row("Mode", Button::Mode, single_active_gamepad_config),
                    header("Action Pad"),
                    button_mapper_row("North", Button::North, single_active_gamepad_config),
                    button_mapper_row("West", Button::West, single_active_gamepad_config),
                    button_mapper_row("East", Button::East, single_active_gamepad_config),
                    button_mapper_row("South", Button::South, single_active_gamepad_config),
                    header("Sticks"),
                    button_mapper_row(
                        "Left Stick Press",
                        Button::LeftThumb,
                        single_active_gamepad_config
                    ),
                    button_mapper_row(
                        "Right Stick Press",
                        Button::RightThumb,
                        single_active_gamepad_config
                    ),
                    header("Triggers"),
                    button_mapper_row(
                        "Left Bumper",
                        Button::LeftTrigger,
                        single_active_gamepad_config
                    ),
                    button_mapper_row(
                        "Left Trigger",
                        Button::LeftTrigger2,
                        single_active_gamepad_config
                    ),
                    button_mapper_row(
                        "Right Bumper",
                        Button::RightTrigger,
                        single_active_gamepad_config
                    ),
                    button_mapper_row(
                        "Right Trigger",
                        Button::RightTrigger2,
                        single_active_gamepad_config
                    ),
                    header("D-Pad"),
                    button_mapper_row("Up", Button::DPadUp, single_active_gamepad_config),
                    button_mapper_row("Left", Button::DPadLeft, single_active_gamepad_config),
                    button_mapper_row("Right", Button::DPadRight, single_active_gamepad_config),
                    button_mapper_row("Down", Button::DPadDown, single_active_gamepad_config),
                    header("Misc."),
                    button_mapper_row("C Button", Button::C, single_active_gamepad_config),
                    button_mapper_row("Z Button", Button::Z, single_active_gamepad_config),
                ]
                .spacing(5)
            )
            .height(Length::Fill),
            row![activate, deactivate, handler_text].spacing(10),
            row![save_profile, load_profile].spacing(10),
        ]
        .spacing(5)
        .height(Length::Fill)
        .into()
    }
}
