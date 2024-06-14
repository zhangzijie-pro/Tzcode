use tauri::command;
use tauri::Manager;

#[command]
pub async fn close_init(window: tauri::Window) {
    // 关闭初始屏幕
    if let Some(init) = window.get_window("init") {
    init.close().unwrap();
    }
    // 显示主窗口
    window.get_window("main").unwrap().show().unwrap();
}