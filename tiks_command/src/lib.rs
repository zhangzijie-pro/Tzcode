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

pub mod init_shell{
    use std::collections::HashMap;

    use crate::commands::command::{and, pipe, priority_run};
    use crate::history_push;
    use crate::process::{process::ProcessState, RUNNING_P};
    use crate::root::SessionContext;
    use crate::run::run;
    use crate::set::set::get_last;


    pub fn init_shell(input: Option<String>,session_context: &mut SessionContext) -> Option<String>{
        // init an Editor
        let mut hash = HashMap::new();
        hash.insert(208, ("bash".to_owned(),ProcessState::Running));
        hash.insert(210, ("cmd".to_owned(),ProcessState::Running));
    
        RUNNING_P.lock().unwrap().push(hash);
        loop {
            match input {
                 Some(ref line)=> {
                    if line.trim().is_empty() {
                        continue;
                    }
                    // add line in lazy HISTORY
                    history_push(line.clone());
                    
                    if line.parse::<usize>().is_ok() {
                        let (_i, res) = get_last(line.parse::<usize>().unwrap());
                        match res {
                            Some(command) => {
                                let args: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
                                run(args, session_context);
                            }
                            None => {
                                continue;
                            }
                        }
                    } else {
                        let args: Box<Vec<String>> = Box::new(line.split_whitespace().map(|s| s.to_string()).collect());
                        if args.contains(&"&&".to_string()) {
                            let mut output = format!("");
                            let res = and(*args, session_context);
                            for (t,r) in res{
                                let str = format!("Done[{t}]: \n{r}");
                                output.push_str(&str);
                            }
                            return Some(output);
                        } else if args.contains(&"|".to_string()) {
                            let res = pipe(*args).unwrap();
                            let str = format!("{}",res.1);
                            return Some(str);
                        } else if args.contains(&"&".to_string()) {
                            let mut output = format!("");
                            let res = priority_run(*args, session_context);
                            for (t,r) in res{
                                let str = format!("Done[{t}]: \n{r}");
                                output.push_str(&str);
                            }
                            return Some(output);
                        } else {
                            let (t,res) = run(*args, session_context);
                            let str = format!("Done[{t}]: \n{res}");
                            return Some(str);
                        }
                        
                    }
                }
                None =>{
                    continue;
                }
            }
        }
    }
}