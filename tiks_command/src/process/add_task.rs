use crate::priority::CommandPriority;
use crate::signal::Semaphore;

use super::thread::Thread;
use super::thread::ThreadControlBlock;
use super::process::Process;
use super::process::ProcessManager;


pub fn add_command_to_thread(tid: usize, name: String,priority: CommandPriority,tcb:&mut ThreadControlBlock){
    let thread = Thread::new(tid, name, priority);
    tcb.add_thread(thread)
}


pub fn add_thread_to_process(pid:usize,name: String,tcb:ThreadControlBlock,semaphore: Semaphore,pcb:&mut ProcessManager){
    let process = Process::new(pid, &name,tcb,semaphore);
    pcb.add_process(process)
}