#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use omg_cool::{log::Log, config::Config};

mod ui;
mod background;

fn main() {
    Log::create_log_dir();
    Config::set_panic_hook();

    std::thread::spawn(||{
        background::start();
    });
    
    ui::start();
    
}
