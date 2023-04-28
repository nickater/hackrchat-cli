use inquire::{InquireError, Select};

pub fn display_welcome_message() {
    println!("Welcome to HackrChat!");
}

pub fn display_main_menu(options: Vec<&str>) -> &str {
    let ans: Result<&str, InquireError> = Select::new("Chat Menu", options).prompt();

    match ans {
        Ok(choice) => choice,
        Err(_) => "Exit",
    }
}
