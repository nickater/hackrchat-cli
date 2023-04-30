pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub sender_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub content: String,
}

impl Message {
    pub fn new(
        id: i32,
        chat_id: i32,
        sender_id: String,
        created_at: String,
        updated_at: String,
        content: String,
    ) -> Self {
        Self {
            id,
            chat_id,
            sender_id,
            created_at,
            updated_at,
            content,
        }
    }
}

pub struct MessageForm {
    pub chat_id: i32,
    pub sender_id: String,
    pub content: String,
}

impl MessageForm {
    pub fn new(chat_id: i32, sender_id: String, content: String) -> Self {
        Self {
            chat_id,
            sender_id,
            content,
        }
    }
}
