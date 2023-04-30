use crate::models::credential::Credential;
use crate::models::user::User;
use std::error::Error;

pub fn login_user(credentials: Credential) -> Result<User, Box<dyn Error>> {
    println!("Login user: {}", credentials.username);

    // TODO: Implement login logic
    Ok(User::new(1, credentials.username))
}

pub mod settings;
