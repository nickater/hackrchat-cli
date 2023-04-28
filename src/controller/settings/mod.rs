use crate::{service::settings, ui, utils::clear};

pub fn settings_controller() {
    let mut is_running = true;

    while is_running {
        let options: Vec<&str> = vec!["Remember Me", "Back"];
        let choice = ui::main::display_main_menu(options);

        match choice {
            "Remember Me" => settings::remember_me(),
            "Back" => is_running = false,
            _ => (),
        };
        clear();
    }
}
