// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler};

mod page;
mod tests;
mod init;
mod extension;
mod splashscreen;

use std::process::Command;
use std::sync::Arc;
use tauri::Manager;
use std::sync::Mutex;
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

  let python_server = Command::new("python")
  .arg("../py/server.py")
  .spawn()
  .expect("Failed to start Python server");

// Use Arc and Mutex to share the Python server process between threads
  let python_server = Arc::new(Mutex::new(python_server));

  
  tauri::Builder::default()
    .setup({
      let python_server = Arc::clone(&python_server);
      move |app| {
          let main_window = app.get_window("main").unwrap();
          main_window.on_window_event({
              let python_server = Arc::clone(&python_server);
              move |event| {
                  if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                      let mut server = python_server.lock().unwrap();
                      server.kill().expect("Failed to kill Python server");
                      api.prevent_close();
                  }
              }
          });
          Ok(())
      }
    })
    .invoke_handler(generate_handler![open_setting,open_new_window,read_directory, read_file, whoami_tauri,pwd_tauri,run_command,close_init,open_workspace,read_workspace_config,write_workspace_config,get_file_language,write_file,get_all_file,get_files_with_pattern,find_file,create_dir,create_file])
    .run(generate_context!())
    .expect("error while running tauri application");
}
