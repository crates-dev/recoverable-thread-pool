use super::r#type::Worker;
use crate::thread_pool::r#type::ThreadPoolJob;
use recoverable_spawn::*;
use std::{
    sync::{Arc, Mutex, mpsc::Receiver},
    thread::spawn,
};

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<ThreadPoolJob>>>) -> Option<Worker> {
        spawn(|| {
            let _ = sync::recoverable_spawn(move || {
                loop {
                    if let Ok(receiver_lock) = receiver.lock() {
                        if let Ok(job) = receiver_lock.recv() {
                            let _ = sync::recoverable_spawn(job);
                        }
                    }
                }
            });
        });
        return Some(Worker { id });
    }
}
