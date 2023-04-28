use crate::models::{Credentials, User};
use std::error::Error;

pub fn login_user(credentials: Credentials) -> Result<User, Box<dyn Error>> {
    println!("Login user: {}", credentials.username);

    // TODO: Implement login logic
    Ok(User::new(1, credentials.username))
}

pub mod settings;
