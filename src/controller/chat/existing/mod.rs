use std::{env, fs};

use super::{existing, new};
use crate::{ui, utils::clear};

pub fn existing_chat_controller() {
    let mut is_running = true;
    // fetch exiting chats from database and put them in a vector
    while is_running {
        let options: Vec<&str> = vec!["nater", "gus22", "staceyk"];
        let choice = ui::chat::chat_menu(options);
        if let Ok(home_dir) = env::var("HOME") {
            fs::write(home_dir + "/.hcrc", choice.to_string()).expect("Unable to write to file");
        }
    }
}
