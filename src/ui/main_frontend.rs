use crate::ui::key_press_frontend::KeyPressWindow;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use gilrs::{GamepadId, Gilrs};
use iced::alignment::Horizontal;
use iced::{window, Element, Length, Size, Subscription, Task, Vector};
use iced::widget::{button, column, row, text, Column, Container};
use iced::window::{Id, Settings};
use uuid::Uuid;
use crate::backend::config_manager::{GamepadConfig, ProfileConfig};
use crate::backend::controller_handler::handle_controller_input;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Activated(()),
    Deactivate,
    OpenWindow,
    WindowOpened(Id),
    WindowClosed(Id),
}

pub trait Window {
    fn view(&self, id: Id) -> Element<'_, Message>;
}

pub struct Mapper {
    gilrs: Arc<Mutex<Gilrs>>,
    profile_config: ProfileConfig,
    windows: BTreeMap<Id, Box<dyn Window>>,
    is_handler_running: Arc<AtomicBool>
}

impl Mapper {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(Settings::default());

        // TODO: Figure out if there's a better way to handle these errors on startup!
        let gilrs = Arc::new(Mutex::new(Gilrs::new().unwrap()));

        (
            Self {
                gilrs: gilrs.clone(),
                // TODO: Figure out if there's a better way to handle these errors on startup!
                profile_config: ProfileConfig::load(gilrs.clone()).unwrap(),
                windows: BTreeMap::new(),
                is_handler_running: Arc::new(AtomicBool::new(false)),
            },
            open.map(Message::WindowOpened),
        )
    }

    fn get_active_gamepad_config_map(&self) -> HashMap<GamepadId, GamepadConfig> {
        let mut gamepad_config_map: HashMap<GamepadId, GamepadConfig> = HashMap::new();

        for (gamepad_id, gamepad) in self.gilrs.lock().unwrap().gamepads() {
            let gc_search_result = self.profile_config.gamepad_configs().iter().find(|gc| {
                Uuid::from_bytes(gamepad.uuid()) == *gc.uuid()
            });
            if let Some(gc) = gc_search_result {
                gamepad_config_map.insert(gamepad_id, gc.clone());
            }
        }

        gamepad_config_map
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Activate => Task::perform(
                {
                    self.is_handler_running.store(true, Ordering::Relaxed);
                    handle_controller_input(self.gilrs.clone(), self.get_active_gamepad_config_map(), self.is_handler_running.clone())
                },
                Message::Activated,
            ),
            Message::Activated(()) => Task::none(),
            Message::Deactivate => {
                self.is_handler_running.store(false, Ordering::Relaxed);
                Task::none()
            },
            Message::OpenWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::get_position(*last_window)
                    .then(|last_position| {
                        let position = last_position.map_or(
                            window::Position::Default,
                            |last_position| {
                                window::Position::Specific(
                                    last_position + Vector::new(20.0, 20.0),
                                )
                            },
                        );

                        let (_, open) = window::open(Settings {
                            position,
                            max_size: Some(Size::new(400_f32, 200_f32)),
                            ..Settings::default()
                        });

                        open
                    })
                    .map(Message::WindowOpened)
            },
            Message::WindowOpened(id) => {
                if self.windows.is_empty() {
                    self.windows.insert(id, Box::new(MainWindow{}));
                } else {
                    self.windows.insert(id, Box::new(KeyPressWindow{}));
                }

                Task::none()
            },
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }

    pub fn view(&self, window_id: Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            return window.view(window_id);
        }
        text("Error: window_id Not Found, could not load view!").into()
    }
}

struct MainWindow;

impl MainWindow {
    fn get_button_mapper<'b>(label: String) -> Column<'b, Message> {
        column![
            text(label),
            button("InsertValueHere")
        ].width(Length::Fill).align_x(Horizontal::Center)
    }
}

impl Window for MainWindow {
    fn view(&self, _window_id: Id) -> Element<'_, Message> {
        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);
        let window_test = button("Open Window").on_press(Message::OpenWindow);

        column![
            row![
                column![
                    text("D-Pad"),
                    MainWindow::get_button_mapper(String::from("Up")),
                    row![
                        MainWindow::get_button_mapper(String::from("Left")),
                        MainWindow::get_button_mapper(String::from("Right")),
                    ],
                    MainWindow::get_button_mapper(String::from("Down")),
                ].width(Length::Fill).spacing(25),
                column![].width(Length::Fill),
                column![].width(Length::Fill),
            ].width(Length::Fill).height(Length::Fill),
            row![].width(Length::Fill).height(Length::Fill),
            row![
                Container::new(window_test).width(Length::Fill).align_x(Horizontal::Right),
                Container::new(activate).width(Length::Fill).align_x(Horizontal::Right),
                Container::new(deactivate).align_x(Horizontal::Right),
            ].spacing(10).width(Length::Fill).height(Length::Fill)
        ].spacing(10).into()
    }
}
