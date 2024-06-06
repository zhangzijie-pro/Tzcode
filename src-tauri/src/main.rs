// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::Read;
use tauri::{command, generate_context, generate_handler};
use ansi_to_html::convert;

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

/*#[command]
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
  let commands: Vec<&str> = command.split_whitespace().collect();
  let commands_string: Vec<String> = commands.iter().map(|x| x.to_string()).collect();
  let res = run(commands_string, &mut SessionContext::new());
  let mut html_output = convert(&res).expect("Can't INTO");
  html_output = html_output.replace("\n", "<br>");
  html_output
}*/

fn main() {
  tauri::Builder::default()
      .invoke_handler(generate_handler![read_directory, read_file, write_file])
      .run(generate_context!())
      .expect("error while running tauri application");
}
