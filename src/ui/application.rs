use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use gilrs::{Button, GamepadId, Gilrs};
use iced::{keyboard, window, Element, Event, Size, Subscription, Task, Vector};
use iced::widget::{text};
use iced::window::{Id, Settings};
use uuid::Uuid;
use crate::backend::config_manager::{GamepadConfig, ProfileConfig};
use crate::backend::controller_handler::handle_controller_input;
use crate::ui::window::key_press_window::KeyPressWindow;
use crate::ui::window::base::{Window, WindowType};
use crate::ui::window::main_window::MainWindow;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Activated(()),
    Deactivate,
    OpenKeySetWindow(Button),
    WindowOpened(Id, WindowType),
    WindowClosed(Id),
    KeyPressed(keyboard::Key),
    UnsetButton(Button),
}

pub struct Application {
    gilrs: Arc<Mutex<Gilrs>>,
    current_btn_to_bind: Option<Button>,
    profile_config: ProfileConfig,
    windows: BTreeMap<Id, Box<dyn Window>>,
    is_handler_running: Arc<AtomicBool>,
}

impl Application {
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
            open.map(|id| Message::WindowOpened(id, WindowType::Main)),
        )
    }

    fn get_active_gamepad_config_map(&self) -> HashMap<GamepadId, GamepadConfig> {
        // TODO: This needs to essentially be removed in favor of sharing the ProfileConfig between
        //  the window and the controller handler!
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
        self.windows.values().any(|window| window.window_type() == WindowType::KeyPress)
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
            Message::OpenKeySetWindow(btn) => {
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
                    .map(|id| Message::WindowOpened(id, WindowType::KeyPress))
            },
            Message::WindowOpened(id, window_type) => {
                if window_type == WindowType::Main {
                    self.windows.insert(id, Box::new(MainWindow{}));
                } else if window_type == WindowType::KeyPress {
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
                    let key_press_window = self.windows.iter().find(|(_, window)| {
                        window.window_type() == WindowType::KeyPress
                    });
                    if let Some((id, _)) = key_press_window {
                        return window::close(*id);
                    }
                }
                Task::none()
            },
            Message::UnsetButton(btn) => {
                self.profile_config.unset_key_to_all(btn);
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
            return window.view(self.get_active_gamepad_config_map());
        }
        text("Error: window_id Not Found, could not load view!").into()
    }
}
