use crate::{models::chat::Chat, ui};

pub fn existing_chat_controller() {
    // fetch existing chats from database and put them in a vector
    loop {
        // let options: Vec<&str> = vec!["nater", "gus22", "staceyk"];

        let chats = vec![
            Chat::new(1, "nater".to_string(), "hello".to_string()),
            Chat::new(1, "gus22".to_string(), "hello".to_string()),
            Chat::new(1, "skater".to_string(), "hello".to_string()),
            Chat::new(1, "cleo".to_string(), "hello".to_string()),
        ];

        let choice = ui::chat::existing_chat_menu(&chats);

        match choice {
            "Back" => break,
            _ => (),
        }

        // fetch chat history from database and put them in a vector
    }
}
