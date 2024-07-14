// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::message;
use tauri::{ generate_handler, WindowEvent};

mod page;
mod tests;
mod init;
mod extension;
mod splashscreen;

use std::process::Command;
use tauri::Manager;
use std::sync::Arc;
use std::sync::Mutex;
use splashscreen::close_init;
use page::{file::*, read_resource_dir};
use page::terimal::{pwd_tauri,whoami_tauri,run_command};
use extension::*;
use page::new_window::*;
use init::user::check_user_active;

fn main() {
  read_resource_dir();
  let context = tauri::generate_context!("./tauri.conf.json");

  let response = check_user_active().is_ok();

  if !response{
    #[cfg(target_os="windows")]
    Command::new("../tiks_command/target/release/tiks.exe").status().expect("Failed run");
    #[cfg(not(windows))]
    Command::new("../tiks_command/target/release/tiks").status().expect("Failed run");
  }

  let python_server = Command::new("python")
  .arg("../py/server.py")
  .spawn()
  .expect("Failed to start Python server");

  let python_server = Arc::new(Mutex::new(python_server));
  
  tauri::Builder::default()
    .setup({
      move |app| {
        let main_window = app.get_window("main").unwrap();

        // 添加更新检查逻辑
        let main_window_clone = main_window.clone();
        let main_window_clone_2 = main_window_clone.clone();

        let app_handle = app.handle().clone();

        let app_handle_2 = app_handle.clone();

        tauri::async_runtime::spawn(async move {
          match app_handle.updater().check().await {
            Ok(update) => {
              if update.is_update_available() {
                message(Some(&main_window_clone), "Update Available", "A new version is available. It will be installed when you restart the application.");
                update.download_and_install().await.unwrap();
              }
            },
            Err(e) => {
              eprintln!("Failed to check for updates: {}", e);
            }
          }
        });


        main_window.on_window_event({
          move |event| {
            let app_handle = app_handle_2.clone();
            if let WindowEvent::CloseRequested { api, .. } = event {
              let mut server = python_server.lock().unwrap();
              server.kill().expect("Failed to kill Python server");
              api.prevent_close();

              // 提示用户是否要更新并关闭应用
              tauri::api::dialog::ask(
                Some(&main_window_clone_2),
                "Confirm Exit",
                "Do you want to update and close the application?",
                move |response| {
                  if response {
                    tauri::async_runtime::spawn(async move {
                      app_handle.updater().should_install(|_current, _latest| true);
                    });
                  } else {
                    // 仅关闭应用而不更新
                    app_handle.exit(0);
                  }
                },
              );
            }
          }
        });
        Ok(())
      }
    })
    .invoke_handler(generate_handler![open_setting,open_new_window,read_directory, read_file, whoami_tauri,pwd_tauri,run_command,close_init,open_workspace,read_workspace_config,write_workspace_config,get_file_language,write_file,get_all_file,get_files_with_pattern,find_file,create_dir,create_file])
    .run(context)
    .expect("error while running tauri application");
}
