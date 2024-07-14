pub mod file;
pub mod terimal;

pub mod new_window{
    use tauri::{WindowBuilder, WindowUrl};
    use tauri::command;
    #[command]
    pub async fn open_new_window(app_handle: tauri::AppHandle){
        let _new_window = WindowBuilder::new(
            &app_handle, 
            "new_window", 
            WindowUrl::App("index.html".into())
        )
        .title("Tzcode")
        .fullscreen(false)
        .resizable(true)
        .decorations(false)
        .inner_size(800.0, 600.0)
        .visible(true)
        .center()
        .build()
        .expect("Failed to build window");
    }

    #[command]
    pub async fn open_setting(app_handle: tauri::AppHandle){
        let _setting = WindowBuilder::new(
            &app_handle,
            "Setting", 
            WindowUrl::App("setting.html".into())
        )
        .title("setting")
        .fullscreen(false)
        .resizable(true)
        .decorations(true)
        .inner_size(300.0, 250.0)
        .visible(true)
        .center()
        .build()
        .expect("Failed to build window");
    }
}

use tauri::api::path::resource_dir;
use tauri::Env;
use std::path::PathBuf;

pub fn read_resource_dir(){
    let context = tauri::generate_context!("./tauri.conf.json");
    if let Some(resource) = resource_dir(context.package_info(), &Env::default()){
        let mut file_path = PathBuf::from(resource);
        file_path.push("_up_/py");
        println!("{:#?}",file_path);
    }
}