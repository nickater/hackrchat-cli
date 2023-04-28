use inquire::{InquireError, Select};

pub fn display_settings_menu(options: Vec<&str>) -> &'static str {
    let ans: Result<&str, InquireError> = Select::new("Settings Menu", options).prompt();

    match ans {
        Ok(choice) => choice,
        Err(_) => "Exit",
    }
}
