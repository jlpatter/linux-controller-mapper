use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use iced::alignment::Horizontal;
use iced::{Length, Task};
use iced::widget::{button, column, row, text, Column, Container};
use crate::backend::controller_handler::handle_controller_input;

#[derive(Clone, Debug)]
pub enum Message {
    Activate,
    Activated(()),
    Deactivate,
}

#[derive(Default)]
pub struct Mapper {
    is_handler_running: Arc<AtomicBool>
}

impl Mapper {
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
        }
    }

    fn get_button_mapper<'b>(label: String) -> Column<'b, Message> {
        column![
            text(label),
            button("InsertValueHere")
        ].width(Length::Fill).align_x(Horizontal::Center)
    }

    pub fn view(&self) -> Column<'_, Message> {
        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);

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
                Container::new(activate).width(Length::Fill).align_x(Horizontal::Right),
                Container::new(deactivate).align_x(Horizontal::Right),
            ].spacing(10).width(Length::Fill).height(Length::Fill)
        ].spacing(10)
    }
}
