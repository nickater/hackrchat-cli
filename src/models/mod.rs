pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct User {
    pub id: i32,
    pub username: String,
}

impl User {
    pub fn new(id: i32, username: String) -> Self {
        Self { id, username }
    }
}
