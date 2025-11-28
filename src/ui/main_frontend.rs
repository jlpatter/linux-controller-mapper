use iced::Task;
use iced::widget::{button, row, text, Row};
use crate::backend::controller_handler::handle_controller_input;

#[derive(Clone, Debug)]
pub enum Message {
    Increment,
    Decrement,
    Activate,
    Activated(()),
}

#[derive(Default)]
pub struct Mapper {
    value: i64,
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
                handle_controller_input(),
                Message::Activated,
            ),
            Message::Activated(()) => Task::none(),
        }
    }

    pub fn view(&self) -> Row<Message> {
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(self.value);

        let activate = button("Activate").on_press(Message::Activate);

        // The layout
        row![decrement, counter, increment, activate]

    }
}
