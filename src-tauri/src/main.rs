// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::Read;
use tauri::{command, generate_context, generate_handler, Menu, MenuItem, Submenu};
use ansi_to_html::convert;
use tiks_command::{pwd, whoami, SESSION};
use tiks_command::init_shell::init_shell;

// 读取目录内容
#[command]
fn read_directory(path: String) -> Result<Vec<(String, bool)>, String> {
  match fs::read_dir(&path) {
      Ok(entries) => {
          let mut files = vec![];
          for entry in entries {
              let entry = entry.map_err(|e| e.to_string())?;
              let file_type = entry.file_type().map_err(|e| e.to_string())?;
              files.push((
                  entry.file_name().to_string_lossy().into_owned(),
                  file_type.is_dir(),
              ));
          }
          Ok(files)
      }
      Err(err) => Err(err.to_string()),
  }
}

// 读取文件内容
#[command]
fn read_file(path: String) -> Result<String, String> {
  let mut file = File::open(&path).map_err(|e| e.to_string())?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
  Ok(contents)
}

// 写入文件内容
#[command]
fn write_file(path: String, contents: String) -> Result<(), String> {
  fs::write(&path, contents).map_err(|e| e.to_string())
}

#[command]
fn whoami_tauri() -> String{
  let mut session_lock = SESSION.lock().unwrap();
  let session = &mut *session_lock;
  whoami(session).expect("Can't found").1
}

#[command]
fn pwd_tauri() -> String{
  pwd().expect("Can't found").1
}

#[command]
fn run_command(command: String) -> String {
  let mut session_lock = SESSION.lock().unwrap();
  let session = &mut *session_lock;
  let res = init_shell(Some(command), session).unwrap();
  let mut html_output = convert(&res).expect("Can't INTO");
  html_output = html_output.replace("\n", "<br>");
  html_output
}

fn main() {
  let submenu = Submenu::new("Actions", Menu::new());
  let menu = Menu::new().add_submenu(submenu.clone()).add_native_item(MenuItem::Copy).add_submenu(submenu);
  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(generate_handler![read_directory, read_file, write_file,whoami_tauri,pwd_tauri,run_command])
    .run(generate_context!())
    .expect("error while running tauri application");
}
