use std::fs;
use std::fs::File;
use std::io::Read;
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
fn _write_file(path: String, contents: String) -> Result<(), String> {
  fs::write(&path, contents).map_err(|e| e.to_string())
}