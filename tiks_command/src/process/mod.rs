use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use process::ProcessState;

use self::thread::ThreadControlBlock;

pub mod process;
pub mod add_task;
pub mod thread;

lazy_static!{
    pub static ref RUNNING_P: Mutex<Vec<HashMap<usize,(String,ProcessState)>>> = Mutex::new(Vec::new());
}

pub fn sleep(tcb:&mut ThreadControlBlock,time: usize) -> String{
    tcb.sleep_threads(time);
    String::from("sleep...")
}

pub fn ps() -> String{
    let mut output = format!(" ");
    let s = RUNNING_P.lock().unwrap();
    for map in s.iter() {
        for (pid, (name, state)) in map {
            output.push_str(&format!("PID: {}  Name: {}  State: {:?} \r\n", pid, name, state));
        }
    }
    output
}