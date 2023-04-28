use std::{env, fs};

pub fn remember_me() {
    println!("Remember me");
    if let Ok(home_dir) = env::var("HOME") {
        fs::write(home_dir + "/.hcrc", "REMEMBER_ME=true").expect("Unable to write to file");
    }
}
