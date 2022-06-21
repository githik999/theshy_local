#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::time;
use std::{process::Command, thread::sleep};

use lucian::{log::Log, server::Server};

mod ui;
mod background;

fn main() {
    Log::create_log_dir();
    Server::set_panic_hook();

    std::thread::spawn(||{
        background::start();
    });
    sleep(time::Duration::from_millis(200));
    Command::new("cmd.exe").arg("/C").arg("start").arg("https://google.com").spawn().unwrap();
    ui::start();
    
}
