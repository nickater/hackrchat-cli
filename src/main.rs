use utils::clear;

use crate::ui::main::display_welcome_message;

mod controller;
mod models;
mod service;
mod ui;
mod utils;

fn main() {
    clear();
    display_welcome_message();
    let user = controller::auth::main_auth_controller();
    if user.is_none() {
        return;
    }
    clear();

    if let Some(_user) = user {
        controller::main::main_controller();
    }
}
