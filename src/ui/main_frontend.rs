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

    pub fn view(&self) -> Column<Message> {
        let activate = button("Activate").on_press(Message::Activate);
        let deactivate = button("Deactivate").on_press(Message::Deactivate);

        column![
            row![
                column![
                    text("D-Pad"),
                    Container::new(button("Up")).width(Length::Fill).align_x(Horizontal::Center),
                    row![
                        Container::new(button("Left")).width(Length::Fill),
                        Container::new(button("Right")).width(Length::Fill).align_x(Horizontal::Right)
                    ],
                    Container::new(button("Down")).width(Length::Fill).align_x(Horizontal::Center)
                ].width(Length::FillPortion(1)).spacing(25),
                column![].width(Length::FillPortion(1)),
                column![].width(Length::FillPortion(1)),
            ].width(Length::Fill).height(Length::Fill),
            row![].width(Length::Fill).height(Length::Fill),
            row![
                Container::new(activate).width(Length::Fill).align_x(Horizontal::Right),
                Container::new(deactivate).align_x(Horizontal::Right),
            ].spacing(10).width(Length::Fill).height(Length::Fill)
        ].spacing(10)
    }
}
