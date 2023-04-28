use inquire::{InquireError, Password, Select, Text};

use crate::models::Credentials;

pub fn login() -> Credentials {
    let credentials = fetch_credentials();
    println!("Login");
    credentials
}

pub fn register() -> Credentials {
    let credentials = fetch_credentials();
    println!("Register");
    credentials
}

fn fetch_credentials() -> Credentials {
    let username = fetch_username();
    let password = fetch_password();
    Credentials::new(username, password)
}

fn fetch_username() -> String {
    if let Ok(username) = Text::new("Username").prompt() {
        return username;
    } else {
        fetch_username()
    }
}

fn fetch_password() -> String {
    if let Ok(password) = Password::new("Password").prompt() {
        return password;
    } else {
        fetch_password()
    }
}

pub fn menu(options: Vec<&str>) -> &str {
    let ans: Result<&str, InquireError> = Select::new("", options).prompt();

    match ans {
        Ok(choice) => choice,
        Err(_) => "Exit",
    }
}
