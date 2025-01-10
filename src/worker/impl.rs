use super::r#type::Worker;
use crate::thread_pool::r#type::ThreadPoolJob;
use recoverable_spawn::*;
use std::sync::{mpsc::Receiver, Arc, Mutex};

impl Worker {
    #[inline]
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<ThreadPoolJob>>>) -> Worker {
        let thread: JoinHandle<()> = recoverable_spawn(move || loop {
            if let Ok(receiver_lock) = receiver.lock() {
                if let Ok(job) = receiver_lock.recv() {
                    job();
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
