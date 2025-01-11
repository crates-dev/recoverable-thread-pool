use super::r#type::Worker;
use crate::thread_pool::r#type::ThreadPoolJob;
use recoverable_spawn::*;
use std::sync::{mpsc::Receiver, Arc, Mutex};

impl Worker {
    #[inline]
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<ThreadPoolJob>>>) -> Option<Worker> {
        recoverable_spawn(move || loop {
            if let Ok(receiver_lock) = receiver.lock() {
                if let Ok(job) = receiver_lock.recv() {
                    let _ = run_function(job);
                }
            }
        });
        return Some(Worker { id });
    }
}
