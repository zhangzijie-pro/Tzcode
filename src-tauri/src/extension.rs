use tauri::command;
use std::fs;


#[command]
pub fn get_all_file(origin_path:String) -> Result<Vec<String>,String>{
    match std::fs::read_dir(&origin_path){
        Ok(entries) => {
        let mut res = vec![];
        for i in entries{
            let entry = i.map_err(|e| e.to_string())?;
            if entry.path().is_file(){
                res.push(entry.file_name().to_string_lossy().into_owned())
            }
        }
            Ok(res)
        },
        Err(err)=>Err(err.to_string()),
    }
}

#[command]
pub fn find_file(pattern: String,file: Vec<String>) -> Vec<String>{
    file.into_iter()
        .filter(|file| file.contains(&pattern))
        .collect()
}

use std::collections::HashMap;
use std::io::{self, BufRead};
#[command]
pub fn get_files_with_pattern(origin_path: String, pattern: String) -> Result<Vec<HashMap<String, Vec<String>>>, String> {
    let mut result = Vec::new();

    let entries = fs::read_dir(&origin_path).map_err(|e| e.to_string())?;
    
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        // 只处理文件
        if path.is_file() {
            let file_name = path.file_name()
                                .and_then(|os_str| os_str.to_str())
                                .map(String::from)
                                .ok_or("Failed to get file name")?;
            
            // 打开文件并逐行读取
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            let reader = io::BufReader::new(file);
            
            let mut matching_lines = vec![];
            for (line_number, line) in reader.lines().enumerate() {
                let line = line.map_err(|e| e.to_string())?;
                if line.contains(&pattern) {
                    matching_lines.push(format!("{}: {}", line_number + 1, line));
                }
            }
            
            // 如果有匹配的行，则将其添加到结果中
            if !matching_lines.is_empty() {
                let mut file_map = HashMap::new();
                file_map.insert(file_name, matching_lines);
                result.push(file_map);
            }
        }
    }

    Ok(result)
}