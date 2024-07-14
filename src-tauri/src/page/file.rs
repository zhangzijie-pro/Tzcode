use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tauri::command;

// 读取目录内容
#[command]
pub fn read_directory(path: String) -> Result<Vec<(String, bool)>, String> {
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
pub fn read_file(path: String) -> Result<String, String> {
  let mut file = File::open(&path).map_err(|e| e.to_string())?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
  Ok(contents)
}

// 写入文件内容
#[command]
pub fn write_file(path: String, content: String) -> Result<(), String> {
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())
}

use tauri::api::dialog::FileDialogBuilder;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn turn_pathbuf(value: std::path::PathBuf) -> String {
    value.to_string_lossy().into_owned()
}

#[command]
pub fn open_workspace() -> Result<String, String> {
    let result = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result);

    FileDialogBuilder::new().pick_folder(move |fold_path| {
        let mut result = result_clone.lock().unwrap();
        *result = Some(match fold_path {
            Some(value) => Ok(turn_pathbuf(value)),
            None => Err("Can't find folder".to_string()),
        });
    });
    for _ in 0..100 {
      thread::sleep(Duration::from_millis(50));
      let result = result.lock().unwrap();
      if result.is_some() {
          println!("{:?}",result.clone().unwrap());
          return result.clone().unwrap();
      }
  }

  Err("Can't find folder".to_string())
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,PartialEq)]
pub struct Config {
    pub(crate) workspace: Vec<String>,
}

const WORK_PATH:&str = "../space/workspace.json";

#[command]
pub fn read_workspace_config() -> Result<Config,String>{
    let output = fs::read_to_string(WORK_PATH).map_err(|err| err.to_string())?;
    let config = serde_json::from_str(&output).map_err(|err| err.to_string())?;
    Ok(config)
}

#[command]
pub fn write_workspace_config(config:Config) -> Result<(),String>{
    let data = serde_json::to_string_pretty(&config).map_err(|err| err.to_string())?;
    fs::write(WORK_PATH, data).map_err(|err| err.to_string())?;
    Ok(())
}

#[command]
pub fn get_file_language(filename:String) -> String{
    let file_spilt:Vec<&str> = filename.split('.').collect();
    if let Some(suffix) = file_spilt.last(){
        match *suffix {
            "txt"=>String::from("text"),
            "rs"=>String::from("rust"),
            "py"=>String::from("python"),
            "js"=>String::from("javascript"),
            "css"=>String::from("css"),
            "html"=>String::from("html"),
            "java"=>String::from("java"),
            "c"=>String::from("C"),
            "php"=>String::from("php"),
            "yaml"=>String::from("yaml"),
            "json"=>String::from("json"),
            "toml"=>String::from("TOML"),
            "md"=>String::from("Markdown"),
            _ => String::from("unkown")
        }
    }else{
        String::from("unknow")
    }
}

#[command]
#[allow(non_snake_case)]
pub fn create_file(fileName: String) -> Result<(), String> {
    match File::create(Path::new(&fileName)) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create file: {}", e)),
    }
}

#[command]
#[allow(non_snake_case)]
pub fn create_dir(fileName: String) -> Result<(),String>{
    let _ = fs::create_dir_all(Path::new(&fileName)).expect("create error");
    Ok(())
}