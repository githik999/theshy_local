#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod background;

fn main() {

    std::thread::spawn(||{
        background::start();
    });
    
    ui::start();
    
}
