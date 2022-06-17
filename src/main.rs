#![windows_subsystem = "windows"]

mod ui;
mod network;

fn main() {

    std::thread::spawn(||{
        network::start();
    });
    
    ui::start();
    
}
