// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler, Menu, Submenu};

mod page;
mod extension;
mod splashscreen;
use splashscreen::close_init;
use page::file::*;
use page::terimal::{pwd_tauri,whoami_tauri,run_command};

fn main() {
  let submenu = Submenu::new("Actions", Menu::new());
  let menu = Menu::new().add_submenu(submenu.clone());
  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(generate_handler![read_directory, read_file, whoami_tauri,pwd_tauri,run_command,close_init])
    .run(generate_context!())
    .expect("error while running tauri application");
}
