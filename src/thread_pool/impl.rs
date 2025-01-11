use super::r#type::ThreadPool;
use crate::{worker::r#type::Worker, SendResult, ThreadPoolJob};
use recoverable_spawn::*;
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

impl ThreadPool {
    #[inline]
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

    #[inline]
    pub fn execute<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            let handle_error_arc: Arc<E> = Arc::new(handle_error);
            let _ = recoverable_spawn_with_error_handle(
                move || {
                    job();
                },
                move |err_str| {
                    let err_string_arc: Arc<String> = Arc::new(err_str.to_string());
                    let handle_error_arc_clone: Arc<E> = Arc::clone(&handle_error_arc);
                    let _ = run_function(move || {
                        let arc_err_string_clone: Arc<String> = Arc::clone(&err_string_arc);
                        handle_error_arc_clone(arc_err_string_clone.as_ref());
                    });
                },
            )
            .join();
        });
        self.sender.send(job_with_handler)
    }
}
