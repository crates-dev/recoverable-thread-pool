use crate::*;
use recoverable_spawn::r#async::*;

/// Async implementation of thread pool operations.
impl ThreadPool {
    /// Executes an async job in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The async function to execute.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn async_execute<F>(&self, job: F) -> SendResult
    where
        F: AsyncRecoverableFunction,
    {
        let job_with_handler = Box::new(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let _ = async_run_function(move || async {
                        job.call().await;
                    })
                    .await;
                });
        });
        self.sender.send(job_with_handler)
    }

    /// Executes an async job with error handling in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The async function to execute.
    /// - `E` - The async error handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn async_execute_with_catch<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: AsyncRecoverableFunction,
        E: AsyncErrorHandlerFunction,
    {
        let job_with_handler = Box::new(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let run_result: AsyncSpawnResult = async_run_function(move || async {
                        job.call().await;
                    })
                    .await;
                    if let Err(err) = run_result {
                        let err_string: String = tokio_error_to_string(&err);
                        let _: AsyncSpawnResult = async_run_error_handle_function(
                            move |err_str| async move {
                                handle_error.call(err_str).await;
                            },
                            Arc::new(err_string),
                        )
                        .await;
                    }
                });
        });
        self.sender.send(job_with_handler)
    }

    /// Executes an async job with error handling and finalization in the thread pool.
    ///
    /// # Arguments
    ///
    /// - `F` - The async function to execute.
    /// - `E` - The async error handler function.
    /// - `L` - The async finally handler function.
    ///
    /// # Returns
    ///
    /// - `SendResult` - Result of the job submission.
    pub fn async_execute_with_catch_finally<F, E, L>(
        &self,
        job: F,
        handle_error: E,
        finally: L,
    ) -> SendResult
    where
        F: AsyncRecoverableFunction,
        E: AsyncErrorHandlerFunction,
        L: AsyncRecoverableFunction,
    {
        let job_with_handler = Box::new(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let run_result: AsyncSpawnResult = async_run_function(move || async {
                        job.call().await;
                    })
                    .await;
                    if let Err(err) = run_result {
                        let err_string: String = tokio_error_to_string(&err);
                        let _: AsyncSpawnResult = async_run_error_handle_function(
                            move |err_str| async move {
                                handle_error.call(err_str).await;
                            },
                            Arc::new(err_string),
                        )
                        .await;
                    }
                    let _: AsyncSpawnResult = async_run_function(move || async {
                        finally.call().await;
                    })
                    .await;
                });
        });
        self.sender.send(job_with_handler)
    }
}
