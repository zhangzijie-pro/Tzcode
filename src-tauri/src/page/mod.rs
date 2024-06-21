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
}