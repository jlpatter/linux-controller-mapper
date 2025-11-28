use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use iced::Task;
use iced::widget::{button, row, text, Row};
use crate::backend::controller_handler::handle_controller_input;

#[derive(Clone, Debug)]
pub enum Message {
    Increment,
    Decrement,
    Activate,
    Activated(()),
    Deactivate,
}

#[derive(Default)]
pub struct Mapper {
    value: i64,
    is_handler_running: Arc<AtomicBool>
}

impl Mapper {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {
                self.value += 1;
                Task::none()
            }
            Message::Decrement => {
                self.value -= 1;
                Task::none()
            }
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
        }
    }

    pub fn view(&self) -> Row<Message> {
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(self.value);

        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);

        // The layout
        row![decrement, counter, increment, activate, deactivate]

    }
}
