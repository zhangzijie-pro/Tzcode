use lazy_static::lazy_static;
use tiks_command::home_dir;

lazy_static!{
    pub static ref USER_FILE:String = {
        let home = home_dir();
        format!("{}/.Tiks/user.ini",home)
    };
}

pub const _COMMAND_PATH:&str = "../target/release/tiks.exe";



pub mod user{
    use std::path::Path;

    use super::USER_FILE;
    pub fn check_user_active() -> Result<(),String>{
        if Path::new(&USER_FILE.as_str()).exists() {
            Ok(())
        } else {
            Err(format!("User file does not exist at path: {}", *USER_FILE))
        }
    }
}

pub mod init_file{
    use std::{fs, io::Write};

    use tauri::command;

    use super::USER_FILE;

    #[command]
    pub fn write_ini(w: String) -> Result<(),String>{
        let ini_path = "../ini/setting.ini";
        let user_tiks_path = USER_FILE.as_str();
        let binding = fs::read(user_tiks_path).unwrap();
        let user_msg = binding.as_slice();
        if let Ok(mut file) = fs::File::open(ini_path){
            let _ = file.write(user_msg);
            let _ = file.write_all(w.as_bytes());
        };

        Ok(())
    }
}