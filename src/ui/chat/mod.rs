use inquire::{InquireError, Select};

use crate::models::chat::Chat;

pub fn chat_menu(options: Vec<&str>) -> &str {
    let ans: Result<&str, InquireError> = Select::new("Chat Menu", options).prompt();

    match ans {
        Ok(choice) => choice,
        Err(_) => "Exit",
    }
}

pub fn existing_chat_menu(chats: &Vec<Chat>) -> &str {
    let options: Vec<&str> = chats.iter().map(|chat| chat.name.as_str()).collect();
    let default_options = vec!["Back"];
    let options = [&options[..], &default_options[..]].concat();
    let ans: Result<&str, InquireError> = Select::new("Chat Menu", options).prompt();

    match ans {
        Ok(choice) => &choice,
        Err(_) => "Exit",
    }
}
