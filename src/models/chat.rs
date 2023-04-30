use super::message::Message;

pub struct Chat {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub messages: Vec<Message>,
}

impl Chat {
    pub fn new(id: i32, name: String, content: String) -> Self {
        Self {
            id,
            name,
            content,
            messages: vec![],
        }
    }
}

pub struct ChatForm {
    pub name: String,
    pub content: String,
}

impl ChatForm {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}
