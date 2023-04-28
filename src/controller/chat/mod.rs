use crate::{ui, utils::clear};
mod existing;
mod new;

pub fn main_chat_controller() {
    let mut is_running = true;
    while is_running {
        let options: Vec<&str> = vec!["New", "Existing", "Back"];
        let choice = ui::chat::chat_menu(options);

        clear();
        match choice {
            "New" => new::new_chat_controller(),
            "Existing" => existing::existing_chat_controller(),
            "Back" => is_running = false,
            _ => (),
        };
    }
}
