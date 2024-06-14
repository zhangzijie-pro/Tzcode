use ansi_to_html::convert;
use tauri::command;
use tiks_command::{pwd, whoami, SESSION};
use tiks_command::init_shell::init_shell;

#[command]
pub fn whoami_tauri() -> String{
  let mut session_lock = SESSION.lock().unwrap();
  let session = &mut *session_lock;
  whoami(session).expect("Can't found").1
}

#[command]
pub fn pwd_tauri() -> String{
  pwd().expect("Can't found").1
}

#[command]
pub fn run_command(command: String) -> String {
  let mut session_lock = SESSION.lock().unwrap();
  let session = &mut *session_lock;
  let res = init_shell(Some(command), session).unwrap();
  let mut html_output = convert(&res).expect("Can't INTO");
  html_output = html_output.replace("\n", "<br>");
  html_output
}