use std::{thread, time::Duration};

use crate::priority::CommandPriority;



#[derive(Clone, Debug)]
pub struct Thread {
    pub tid: usize,
    pub name: String,
    pub priority: CommandPriority,
    pub state: ThreadStatus,
}


#[derive(Debug, Clone)]
pub enum ThreadStatus {
    Running,
    Block,
    Stopped,
}


impl Thread {
    pub fn new(tid: usize, name: String, priority: CommandPriority) -> Self {
        Thread {
            tid,
            name,
            priority,
            state: ThreadStatus::Running,
        }
    }

    pub fn stop(&mut self) {
        self.state = ThreadStatus::Stopped;
    }

    pub fn sleep(&mut self) {
        self.state = ThreadStatus::Block;
    }

    pub fn start(&mut self) {
        self.state = ThreadStatus::Running;
    }

    pub fn status(&self) -> &ThreadStatus {
        &self.state
    }
}


// TCB
#[derive(Clone, Debug)]
pub struct ThreadControlBlock {
    pub threads: Vec<Thread>,
}


#[allow(dropping_references)]
impl Drop for ThreadControlBlock{
    fn drop(&mut self) {
        for i in self.threads.iter_mut(){
            drop(&i);
            i.stop()
        }
    }
}

impl ThreadControlBlock {
    pub fn new() -> ThreadControlBlock {
        ThreadControlBlock {
            threads: Vec::new(),
        }
    }

    pub fn add_thread(&mut self, thread: Thread) {
        self.threads.push(thread)
    }

    pub fn stop_thread(&mut self, tid: usize) {
        if let Some(thread) = self.threads.iter_mut().find(|t| t.tid == tid) {
            thread.stop();
        } else {
            println!("Thread with TID {} not found", tid);
        }
    }

    pub fn start_thread(&mut self, tid: usize) {
        if let Some(thread) = self.threads.iter_mut().find(|t| t.tid == tid) {
            thread.start();
        } else {
            println!("Thread with TID {} not found", tid);
        }
    }

    pub fn sleep_threads(&mut self, time: usize) {
        for thread in &mut self.threads {
            thread.sleep();
            thread::sleep(Duration::from_secs(time as u64));
            thread.start();
        }
    }

    pub fn get_highest_priority_thread(&self) -> Option<usize> {
        self.threads
            .iter()
            .max_by(|a, b| a.priority.cmp(&b.priority))
            .map(|t| t.tid)
    }
}