use crate::*;
use recoverable_spawn::sync::*;

/// Sync implementation of thread pool operations.
impl ThreadPool {
    /// Creates a new thread pool with the specified number of workers.
    ///
    /// # Arguments
    ///
    /// - `usize` - The number of worker threads to create.
    ///
    /// # Returns
    ///
    /// - `ThreadPool` - The new thread pool instance.
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

    /// Executes a synchronous job in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The synchronous function to execute.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn execute<F>(&self, job: F) -> SendResult
    where
        F: RecoverableFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            let _ = run_function(job);
        });
        self.sender.send(job_with_handler)
    }

    /// Executes a synchronous job with error handling in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The synchronous function to execute.
    /// - `E` - The error handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn execute_with_catch<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: RecoverableFunction,
        E: ErrorHandlerFunction,
    {
        let job_with_handler: ThreadPoolJob = Box::new(move || {
            if let Err(err) = run_function(job) {
                let err_string: String = spawn_error_to_string(&err);
                let _ = run_error_handle_function(handle_error, &err_string);
            }
        });
        self.sender.send(job_with_handler)
    }

    /// Executes a synchronous job with error handling and finalization in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The synchronous function to execute.
    /// - `E` - The error handler function.
    /// - `L` - The finally handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
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
            if let Err(err) = run_function(job) {
                let err_string: String = spawn_error_to_string(&err);
                let _ = run_error_handle_function(handle_error, &err_string);
            }
            let _ = run_function(finally);
        });
        self.sender.send(job_with_handler)
    }
}
