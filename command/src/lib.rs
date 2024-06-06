pub mod root;
pub mod commands;
mod set;
pub mod process;
pub mod run;
pub mod signal;
mod priority;
pub mod env;
mod test;
pub mod start;
mod tauri_func;

pub use crate::root::SESSION;
pub use crate::start::*;
pub use crate::tauri_func::run_command;
pub use crate::commands::command::{HISTROY,history_push,pwd,whoami,get_time};
pub use crate::set::set::home_dir;