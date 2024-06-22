use lazy_static::lazy_static;
use tiks_command::home_dir;

lazy_static!{
    pub static ref USER_FILE:String = {
        let home = home_dir();
        format!("{}/.Tiks/tiks",home)
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