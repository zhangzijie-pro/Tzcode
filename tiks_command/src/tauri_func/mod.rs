use crate::root::SessionContext;
use crate::commands::arg::{split, Commands,command_match};
use crate::process::{add_task::*, ps, sleep};
use crate::process::process::ProcessManager;
use crate::process::thread::ThreadControlBlock;
use crate::run::handle_command;
use crate::set::set::error_log;
use crate::signal::semaphore_new;

#[allow(dead_code)]
pub fn string_to_command(string: String) -> Commands{
    Commands::from_string(string)
}

pub fn run_command<T>(input: T,session_context: &mut SessionContext) -> (usize,String)
where Vec<String>: From<T>
{
    let (commands,pid,tid,priority) = handle_command(input.into());
    let semaphore = semaphore_new();
    let mut tcb = ThreadControlBlock::new();
    let mut pcb = ProcessManager::new();


    let (command,_option,arg) = split(commands.clone());

    add_command_to_thread(tid, command.clone(), priority, &mut tcb);
    add_thread_to_process(pid, command.clone(), tcb.clone(), semaphore, &mut pcb);
    
    let priority_tid = tcb.get_highest_priority_thread().unwrap();
    tcb.start_thread(priority_tid);
    pcb.start_process(pid);

    
    match command.as_str(){
        "ps" => {
            pcb.kill(pid);
            tcb.stop_thread(priority_tid);
            (tid,ps())
        }
        "kill" => {
            tcb.stop_thread(priority_tid);
            pcb.kill(pid);
            (tid,pcb.kill(arg[0].parse::<usize>().unwrap()))
        },
        "sleep" => {
            pcb.kill(pid);
            tcb.stop_thread(priority_tid);
            (tid,sleep(&mut tcb, commands.arg[0].parse::<usize>().unwrap()))
        }
        _ =>{
            // start process and thread
            if session_context.user_state.root.check_permission(){
                // Execute root commands
                // Handle commands differently when user is in root mode
                if let Ok(res) = command_match(commands, session_context){
                    let status = res.0.clone();
                    let result = res.1.clone();
                    if status==0{
                        pcb.kill(pid);
                        tcb.stop_thread(priority_tid);
                        (tid,result)
                    }else{
                        error_log(res.1.clone());  
                        (tid,result)
                    }
                }else{(tid,String::from("Error: Not found"))}
            } else if !session_context.user_state.root.check_permission() && !session_context.root.allowed_commands.contains(&commands.command) {
                // Execute normal commands
                // Handle commands normally when user is not in root mode
                if let Ok(res) = command_match(commands, session_context) {
                    let status = res.0.clone();
                    let result = res.1.clone();
                    if status==0{
                        pcb.kill(pid);
                        tcb.stop_thread(priority_tid);
                        (tid,result)
                    }else{
                        error_log(res.1.clone());
                        (tid,result)
                    }
                }else {
                    (tid,String::from("Error: Not found"))
                }
            }else{
                (tid,String::from("Error: Permission not support"))
            }
        }
    }
}