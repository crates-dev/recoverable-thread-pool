use super::r#type::ThreadPool;
use crate::{SendResult, ThreadPoolJob, worker::r#type::Worker};
use recoverable_spawn::*;
use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
};

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<ThreadPoolJob>>> = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let mut id: usize = 0;
        loop {
            if id >= size {
                break;
            }
            let worker: Option<Worker> = Worker::new(id, Arc::clone(&receiver));
            if worker.is_some() {
                workers.push(worker.unwrap_or_default());
                id += 1;
            }
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, job: F) -> SendResult
    where
        F: RecoverableFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            let _ = sync::run_function(job);
        });
        self.sender.send(job_with_handler)
    }

    pub fn execute_with_catch<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            if let Err(err) = sync::run_function(job) {
                let err_string: String = sync::spawn_error_to_string(err);
                let _ = sync::run_error_handle_function(handle_error, &err_string);
            }
        });
        self.sender.send(job_with_handler)
    }

    pub fn execute_with_catch_finally<F, E, L>(
        &self,
        job: F,
        handle_error: E,
        finally: L,
    ) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
        L: RecoverableFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            if let Err(err) = sync::run_function(job) {
                let err_string: String = sync::spawn_error_to_string(err);
                let _ = sync::run_error_handle_function(handle_error, &err_string);
            }
            let _ = sync::run_function(finally);
        });
        self.sender.send(job_with_handler)
    }
}
