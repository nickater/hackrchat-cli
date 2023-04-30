use crate::{models::user::User, ui::auth, utils::clear};

pub fn main_auth_controller() -> Option<User> {
    let options: Vec<&str> = vec!["Login", "Register", "Exit"];

    let choice = auth::menu(options);
    let credentials = match choice {
        "Login" => Some(auth::login()),
        "Register" => Some(auth::register()),
        "Exit" => None,
        _ => None,
    };

    clear();
    if let Some(credentials) = credentials {
        Some(User::new(123, credentials.username))
    } else {
        None
    }
}
