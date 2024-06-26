// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::message;
use tauri::{generate_context, generate_handler};

mod page;
mod tests;
mod init;
mod extension;
mod splashscreen;

use std::process::Command;
use tauri::Manager;
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

  let _python_server = Command::new("python")
  .arg("../py/server.py")
  .spawn()
  .expect("Failed to start Python server");

  
  tauri::Builder::default()
    .setup({
      move |app| {
        let main_window = app.get_window("main").unwrap();

        // 添加更新检查逻辑
        let app_handle = app.handle().clone();
        tauri::async_runtime::spawn(async move {
          match app_handle.updater().check().await {
            Ok(update) => {
              if update.is_update_available() {
                message(Some(&main_window), "Update Available", "A new version is available. It will be installed when you restart the application.");
                update.download_and_install().await.unwrap();
              }
            },
            Err(e) => {
              eprintln!("Failed to check for updates: {}", e);
            }
          }
        });
        Ok(())
      }
    })
    .invoke_handler(generate_handler![open_setting,open_new_window,read_directory, read_file, whoami_tauri,pwd_tauri,run_command,close_init,open_workspace,read_workspace_config,write_workspace_config,get_file_language,write_file,get_all_file,get_files_with_pattern,find_file,create_dir,create_file])
    .run(generate_context!("./tauri.conf.json"))
    .expect("error while running tauri application");
}
