pub struct Credential {
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}
