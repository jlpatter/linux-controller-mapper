mod ui;
mod backend;

use crate::ui::main_frontend::Mapper;

fn main() {
    iced::run("Linux Controller Mapper", Mapper::update, Mapper::view).unwrap();
}
