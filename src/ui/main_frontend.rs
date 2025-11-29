use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use iced::alignment::Horizontal;
use iced::{window, Element, Length, Subscription, Task, Theme, Vector};
use iced::widget::{button, column, row, text, Column, Container, Row};
use iced::window::{Id, Settings};
use crate::backend::controller_handler::handle_controller_input;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Activated(()),
    Deactivate,
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
}

#[derive(Debug)]
struct KeyPressWindow {
    title: String,
    scale_input: String,
    current_scale: f64,
    theme: Theme,
}

#[derive(Default)]
pub struct Mapper {
    other_windows: BTreeMap<window::Id, KeyPressWindow>,
    is_handler_running: Arc<AtomicBool>
}

impl Mapper {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(Settings::default());

        (
            Self {
                other_windows: BTreeMap::new(),
                is_handler_running: Arc::new(AtomicBool::new(false)),
            },
            open.map(Message::WindowOpened),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Activate => Task::perform(
                {
                    self.is_handler_running.store(true, Ordering::Relaxed);
                    handle_controller_input(self.is_handler_running.clone())
                },
                Message::Activated,
            ),
            Message::Activated(()) => Task::none(),
            Message::Deactivate => {
                self.is_handler_running.store(false, Ordering::Relaxed);
                Task::none()
            },
            Message::OpenWindow => {
                // let (_, task) = window::open(Settings::default());
                // task.map(Message::WindowOpened)

                let Some(last_window) = self.other_windows.keys().last() else {
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
                            ..Settings::default()
                        });

                        open
                    })
                    .map(Message::WindowOpened)
            },
            Message::WindowOpened(id) => {
                // TODO: This is how the view of a window is determined, fix it!
                let window = KeyPressWindow::new(self.other_windows.len() + 1);
                self.other_windows.insert(id, window);
                Task::none()
            },
            Message::WindowClosed(id) => {
                self.other_windows.remove(&id);

                if self.other_windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    fn get_button_mapper<'b>(label: String) -> Column<'b, Message> {
        column![
            text(label),
            button("InsertValueHere")
        ].width(Length::Fill).align_x(Horizontal::Center)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.other_windows.get(&window_id) {
            window.view(window_id).into()
        } else {
            let activate = button("Activate").on_press(Message::Activate);
            let deactivate = button("Deactivate").on_press(Message::Deactivate);
            let window_test = button("Open Window").on_press(Message::OpenWindow);

            column![
                row![
                    column![
                        text("D-Pad"),
                        Mapper::get_button_mapper(String::from("Up")),
                        row![
                            Mapper::get_button_mapper(String::from("Left")),
                            Mapper::get_button_mapper(String::from("Right")),
                        ],
                        Mapper::get_button_mapper(String::from("Down")),
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
}

impl KeyPressWindow {
    fn new(count: usize) -> Self {
        Self {
            title: format!("Window_{count}"),
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
            theme: Theme::ALL[count % Theme::ALL.len()].clone(),
        }
    }

    fn view(&self, id: window::Id) -> Row<'_, Message> {
        row![text("This is a new window!")]
    }
}