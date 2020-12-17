use std::mem;
use std::ops::Drop;
use std::thread::{self, JoinHandle};

pub struct Jobber {
    works: Vec<JoinHandle<()>>,
}

impl Jobber {
    pub fn new() -> Jobber {
        Jobber { works: Vec::new() }
    }

    pub fn queue<F>(&mut self, f: F)
    where
        F: FnOnce() -> (),
        F: Send + 'static,
    {
        self.works.push(thread::spawn(f));
    }
}

impl Drop for Jobber {
    fn drop(&mut self) {
        let works = mem::replace(&mut self.works, vec![]);
        for work in works.into_iter() {
            work.join().expect("Failed to exit thread");
        }
    }
}
