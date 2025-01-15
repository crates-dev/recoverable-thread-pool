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
    pub fn execute<F, E, L>(&self, job: F, handle_error: E, finally: L) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
        L: RecoverableFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            let _ = recoverable_spawn_catch_finally(
                move || {
                    job();
                },
                move |err_str| {
                    handle_error(err_str);
                },
                move || {
                    finally();
                },
            )
            .join();
        });
        self.sender.send(job_with_handler)
    }

    #[inline]
    pub fn async_execute<F, E, L>(&self, job: F, handle_error: E, finally: L) -> SendResult
    where
        F: AsyncRecoverableFunction,
        E: AsyncErrorHandlerFunction,
        L: AsyncRecoverableFunction,
    {
        let job_with_handler = Box::new(move || {
            let run_result: AsyncSpawnResult = async_run_function(move || async {
                job.call().await;
            });
            if let Err(err) = run_result {
                let err_string: String = tokio_error_to_string(err);
                let _: AsyncSpawnResult = async_run_error_handle_function(
                    move |err_str| async move {
                        handle_error.call(err_str).await;
                    },
                    Arc::new(err_string),
                );
            }
            let _: AsyncSpawnResult = async_run_function(move || async {
                finally.call().await;
            });
        });
        self.sender.send(job_with_handler)
    }
}
