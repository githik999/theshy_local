#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lucian::{log::Log, server::Server};

mod ui;
mod background;

fn main() {
    Log::create_log_dir();
    Server::set_panic_hook();

    std::thread::spawn(||{
        background::start();
    });
    
    ui::start();
    
}
