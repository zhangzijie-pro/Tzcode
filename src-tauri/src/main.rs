// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler};

mod page;
mod tests;
mod init;
mod extension;
mod splashscreen;

use std::process::Command;
use splashscreen::close_init;
use page::file::*;
use page::terimal::{pwd_tauri,whoami_tauri,run_command};
use extension::*;
use page::new_window::*;
use init::user::check_user_active;

fn main() {
  let response = check_user_active().is_ok();

  if !response{
    #[cfg(target_os="windows")]
    Command::new("../target/release/tiks.exe").status().expect("Failed run");
    #[cfg(not(windows))]
    Command::new("../target/release/tiks").status().expect("Failed run");
  }

  
  tauri::Builder::default()
    .invoke_handler(generate_handler![open_setting,open_new_window,read_directory, read_file, whoami_tauri,pwd_tauri,run_command,close_init,open_workspace,read_workspace_config,write_workspace_config,get_file_language,write_file,get_all_file,get_files_with_pattern,find_file])
    .run(generate_context!())
    .expect("error while running tauri application");
}
