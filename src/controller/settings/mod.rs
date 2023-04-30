use crate::{service::settings, ui, utils::clear};

pub fn settings_controller() {
    loop {
        let options: Vec<&str> = vec!["Remember Me", "Back"];
        let choice = ui::main::display_main_menu(options);

        match choice {
            "Remember Me" => settings::remember_me(),
            "Back" => break,
            _ => break,
        };
        clear();
    }
}
