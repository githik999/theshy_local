#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lucian::log::Log;

mod ui;
mod background;

fn main() {
    Log::init();

    std::thread::spawn(||{
        background::start();
    });
    
    ui::start();
    
}
