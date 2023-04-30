use inquire::{InquireError, Password, Select, Text};

use crate::models::credential::Credential;

pub fn login() -> Credential {
    let credentials = fetch_credentials();
    println!("Login");
    credentials
}

pub fn register() -> Credential {
    let credentials = fetch_credentials();
    println!("Register");
    credentials
}

fn fetch_credentials() -> Credential {
    let username = fetch_username();
    let password = fetch_password();
    Credential::new(username, password)
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
