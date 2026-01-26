use anyhow::Result;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use gilrs::Button;
use iced::{keyboard, window, Element, Event, Size, Subscription, Task, Vector};
use iced::widget::{text};
use iced::window::{Id, Settings};
use crate::backend::config_manager::ProfileConfig;
use crate::backend::controller_handler::handle_controller_input;
use crate::ui::window::key_press_window::KeyPressWindow;
use crate::ui::window::base::{Window, WindowType};
use crate::ui::window::error_window::ErrorWindow;
use crate::ui::window::main_window::MainWindow;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Activated(Result<(), String>),
    Deactivate,
    OpenKeySetWindow(Button),
    WindowOpened(Id, WindowType),
    WindowClosed(Id),
    KeyPressed(keyboard::Key),
    UnsetButton(Button),
    SaveProfile,
    LoadProfile,
}

pub struct Application {
    current_btn_to_bind: Option<Button>,
    profile_config: Arc<Mutex<ProfileConfig>>,
    windows: BTreeMap<Id, Box<dyn Window>>,
    is_handler_running: Arc<AtomicBool>,
    current_error: String,
}

impl Application {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(Settings::default());

        (
            Self {
                current_btn_to_bind: None,
                profile_config: Arc::new(Mutex::new(ProfileConfig::default())),
                windows: BTreeMap::new(),
                is_handler_running: Arc::new(AtomicBool::new(false)),
                current_error: String::new(),
            },
            open.map(|id| Message::WindowOpened(id, WindowType::Main)),
        )
    }

    fn is_key_press_window_open(&self) -> bool {
        self.windows.values().any(|window| window.window_type() == WindowType::KeyPress)
    }

    fn handle_error(&mut self, err: String) -> Task<Message> {
        self.current_error = err;
        let (_, open_task) = window::open(Settings::default());
        open_task.map(|id| Message::WindowOpened(id, WindowType::Error))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Activate => Task::perform(
                {
                    self.is_handler_running.store(true, Ordering::Relaxed);
                    handle_controller_input(self.profile_config.clone(), self.is_handler_running.clone())
                },
                Message::Activated,
            ),
            Message::Activated(result) => {
                if let Err(e) = result {
                    self.is_handler_running.store(false, Ordering::Relaxed);
                    return self.handle_error(e);
                }
                Task::none()
            },
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
                match window_type {
                    WindowType::Main => {
                        self.windows.insert(id, Box::new(MainWindow::new(self.profile_config.clone(), self.is_handler_running.clone())));
                    }
                    WindowType::KeyPress => {
                        self.windows.insert(id, Box::new(KeyPressWindow{}));
                    }
                    WindowType::Error => {
                        self.windows.insert(id, Box::new(ErrorWindow::new(self.current_error.clone())));
                    }
                };

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
                    let mut profile_config = self.profile_config.lock().unwrap();
                    // TODO: Replace self.current_btn_to_bind.unwrap() with a proper check
                    profile_config.insert_key_to_all(self.current_btn_to_bind.unwrap(), key);
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
                let mut profile_config = self.profile_config.lock().unwrap();
                profile_config.unset_key_to_all(btn);
                Task::none()
            },
            Message::SaveProfile => {
                let profile_config = self.profile_config.lock().unwrap();
                profile_config.save().unwrap();
                Task::none()
            },
            Message::LoadProfile => {
                let loaded_profile_config_opt = ProfileConfig::load().unwrap();
                if let Some(loaded_profile_config) = loaded_profile_config_opt {
                    let mut current_profile_config = self.profile_config.lock().unwrap();
                    *current_profile_config = loaded_profile_config;
                }
                Task::none()
            },
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
            return window.view();
        }
        text("Error: window_id Not Found, could not load view!").into()
    }
}
