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
        .decorations(false)
        .inner_size(300.0, 250.0)
        .visible(true)
        .build()
        .expect("Failed to build window");
    }
}