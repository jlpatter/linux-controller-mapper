use crate::ui::key_press_frontend::KeyPressWindow;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use gilrs::{Button, GamepadId, Gilrs};
use iced::{keyboard, window, Element, Event, Length, Size, Subscription, Task, Vector};
use iced::widget::{button, column, row, text, Row};
use iced::window::{Id, Settings};
use serde_json::Value::{Object, String as Json_String};
use uuid::Uuid;
use crate::backend::config_manager::{GamepadConfig, ProfileConfig};
use crate::backend::controller_handler::handle_controller_input;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Activated(()),
    Deactivate,
    OpenWindow(Button),
    WindowOpened(Id),
    WindowClosed(Id),
    KeyPressed(keyboard::Key),
}

pub trait Window {
    fn view(&self, id: Id, active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message>;
}

pub struct Mapper {
    gilrs: Arc<Mutex<Gilrs>>,
    current_btn_to_bind: Option<Button>,
    profile_config: ProfileConfig,
    windows: BTreeMap<Id, Box<dyn Window>>,
    is_handler_running: Arc<AtomicBool>,
}

impl Mapper {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(Settings::default());
        let gilrs = Arc::new(Mutex::new(Gilrs::new().unwrap()));

        (
            Self {
                gilrs: gilrs.clone(),
                current_btn_to_bind: None,
                profile_config: ProfileConfig::load(gilrs.clone()).unwrap(),
                windows: BTreeMap::new(),
                is_handler_running: Arc::new(AtomicBool::new(false)),
            },
            open.map(Message::WindowOpened),
        )
    }

    fn get_active_gamepad_config_map(&self) -> HashMap<GamepadId, GamepadConfig> {
        // TODO: This needs to essentially be removed in favor of sharing the ProfileConfig between
        //  the windows and the controller handler!
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

    fn is_key_press_window_open(&self) -> bool {
        // I wonder if there'd be a less-hacky way to do this...
        self.windows.len() > 1
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
            Message::OpenWindow(btn) => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                self.current_btn_to_bind = Some(btn);

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
            },
            Message::KeyPressed(key) => {
                if key != keyboard::Key::Unidentified {
                    self.profile_config.insert_key_to_all(self.current_btn_to_bind.unwrap(), key);
                    if self.is_key_press_window_open() {
                        return window::close(*self.windows.keys().last().unwrap());
                    }
                }
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![window::close_events().map(Message::WindowClosed)];

        if self.is_key_press_window_open() {
            subs.push(
                iced::event::listen().map(|event| {
                    if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
                        Message::KeyPressed(key)
                    } else {
                        // Ignore other events
                        Message::KeyPressed(keyboard::Key::Unidentified)
                    }
                })
            )
        }

        Subscription::batch(subs)
    }

    pub fn view(&self, window_id: Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            return window.view(window_id, self.get_active_gamepad_config_map());
        }
        text("Error: window_id Not Found, could not load view!").into()
    }
}

struct MainWindow;

impl MainWindow {
    fn get_button_mapper<'b>(label: String, btn: Button, gc_opt: Option<&GamepadConfig>) -> Row<'b, Message> {
        row![
            text(format!("Gamepad Input: {}", label)),
            text(format!(" is currently assigned to: {}", Self::get_str_from_config(gc_opt, &btn))),
            button("Set").on_press(Message::OpenWindow(btn))
        ].width(Length::Fill)
    }

    fn get_str_from_config(gc_opt: Option<&GamepadConfig>, gilrs_btn: &Button) -> String {
        if let Some(gc) = gc_opt {
            if let Some(key) = gc.get_key(gilrs_btn) {
                // TODO: Put in proper error handling!
                if let Object(obj_map) = serde_json::to_value(key).unwrap() {
                    if let Some(key_val) = obj_map.get("Unicode") {
                        if let Json_String(key_str) = key_val {
                            return key_str.clone().to_uppercase();
                        }
                    }
                }
            }
        }
        String::from("None")
    }
}

impl Window for MainWindow {
    fn view(&self, _window_id: Id, active_gamepad_config_map: HashMap<GamepadId, GamepadConfig>) -> Element<'_, Message> {
        let single_active_gamepad_config = active_gamepad_config_map.values().next();

        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);

        column![
            text("D-Pad"),
            MainWindow::get_button_mapper(String::from("Up"), Button::DPadUp, single_active_gamepad_config),
            MainWindow::get_button_mapper(String::from("Left"), Button::DPadLeft, single_active_gamepad_config),
            MainWindow::get_button_mapper(String::from("Right"), Button::DPadRight, single_active_gamepad_config),
            MainWindow::get_button_mapper(String::from("Down"), Button::DPadDown, single_active_gamepad_config),
            row![activate, deactivate],
        ].spacing(10).into()
    }
}
