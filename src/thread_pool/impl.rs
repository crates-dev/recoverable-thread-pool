use super::r#type::ThreadPool;
use crate::worker::r#type::Worker;
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

impl ThreadPool {
    #[inline]
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Box<dyn Fn() + Send>>>> = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    #[inline]
    pub fn execute<F>(&self, job: F)
    where
        F: Fn() + Send + 'static,
    {
        self.sender.send(Box::new(job)).unwrap();
    }
}
