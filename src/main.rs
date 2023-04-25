mod controller;
mod ui;

fn main() {
    controller::auth::hello_from_auth();
    controller::chat::hello_from_chat();
    ui::auth::hello_from_auth_ui();
    ui::chat::hello_from_chat_ui();
}
