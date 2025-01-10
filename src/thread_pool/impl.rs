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
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    #[inline]
    pub fn execute<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
    {
        let arc_job: Arc<F> = Arc::new(job);
        let arc_handle_error: Arc<E> = Arc::new(handle_error);
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            let arc_job_clone: Arc<F> = Arc::clone(&arc_job);
            let arc_handle_error_clone: Arc<E> = Arc::clone(&arc_handle_error);
            let _ = recoverable_spawn_with_error_handle(
                move || {
                    arc_job_clone();
                },
                move |err_string| {
                    let arc_err_string: Arc<String> = Arc::new(err_string.to_string());
                    let arc_handle_error_clone_clone: Arc<E> = Arc::clone(&arc_handle_error_clone);
                    let _ = recoverable_spawn(move || {
                        let arc_err_string_clone: Arc<String> = Arc::clone(&arc_err_string);
                        arc_handle_error_clone_clone(arc_err_string_clone.as_ref());
                    })
                    .join();
                },
            )
            .join();
        });
        self.sender.send(job_with_handler)
    }
}
