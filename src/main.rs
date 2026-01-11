mod ui;
mod backend;

use crate::ui::main_frontend::Mapper;

fn main() {
    iced::daemon("Linux Controller Mapper", Mapper::update, Mapper::view).subscription(Mapper::subscription).run_with(Mapper::new).unwrap();
}
