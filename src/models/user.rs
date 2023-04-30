pub struct User {
    pub id: i32,
    pub username: String,
}

impl User {
    pub fn new(id: i32, username: String) -> Self {
        Self { id, username }
    }
}

pub struct UserForm {
    pub username: String,
}

impl UserForm {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}
