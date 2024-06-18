// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler};

mod page;
mod extension;
mod splashscreen;
use splashscreen::close_init;
use page::file::*;
use page::terimal::{pwd_tauri,whoami_tauri,run_command};

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![read_directory, read_file, whoami_tauri,pwd_tauri,run_command,close_init,open_workspace,read_workspace_config,write_workspace_config,get_file_language,write_file])
    .run(generate_context!())
    .expect("error while running tauri application");
}
