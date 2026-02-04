mod backend;
mod ui;
mod utils;

use crate::ui::application::Application;

fn main() {
    iced::daemon(Application::new, Application::update, Application::view)
        .title("Linux Controller Mapper")
        .subscription(Application::subscription)
        .run()
        .unwrap();
}
