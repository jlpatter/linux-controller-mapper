mod ui;
mod backend;

use crate::ui::application::Application;

fn main() {
    iced::daemon("Linux Controller Mapper", Application::update, Application::view).subscription(Application::subscription).run_with(Application::new).unwrap();
}
