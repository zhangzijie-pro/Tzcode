use std::sync::{Arc, Mutex, Condvar};

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Semaphore{
    count: usize,
    condvar: Arc<(Mutex<usize>,Condvar)>
}

impl Semaphore{
    pub fn new(count: usize) -> Self{
        let mutex = Mutex::new(count);
        let condvar = Arc::new((mutex,Condvar::new()));
        Semaphore{
            count,
            condvar
        }
    }

    pub fn acquire(&self){
        let (mutex,condvar) = &*self.condvar;
        let mut count = mutex.lock().unwrap();
        while *count == 0{
            count = condvar.wait(count).unwrap()
        }
        *count-=1
    }

    pub fn release(&self){
        let (mutex,condvar) = &*self.condvar;
        let mut count = mutex.lock().unwrap();
        *count+=1;
        condvar.notify_one()
    }
}

pub fn semaphore_new() -> Semaphore{
    Semaphore::new(2)
}