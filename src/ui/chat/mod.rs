use inquire::{InquireError, Select};

pub fn chat_menu(options: Vec<&str>) -> &str {
    let ans: Result<&str, InquireError> = Select::new("Chat Menu", options).prompt();

    match ans {
        Ok(choice) => choice,
        Err(_) => "Exit",
    }
}
