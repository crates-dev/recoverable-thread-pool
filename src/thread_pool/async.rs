use crate::*;
use recoverable_spawn::*;
use std::sync::Arc;

impl ThreadPool {
    #[inline]
    pub fn async_execute<F>(&self, job: F) -> SendResult
    where
        F: AsyncRecoverableFunction,
    {
        let job_with_handler = Box::new(move || {
            let _ = run_async_function(move || async {
                job.call().await;
            });
        });
        self.sender.send(job_with_handler)
    }

    #[inline]
    pub fn async_execute_with_catch<F, E>(&self, job: F, handle_error: E) -> SendResult
    where
        F: AsyncRecoverableFunction,
        E: AsyncErrorHandlerFunction,
    {
        let job_with_handler = Box::new(move || {
            let run_result: AsyncSpawnResult = run_async_function(move || async {
                job.call().await;
            });
            if let Err(err) = run_result {
                let err_string: String = tokio_error_to_string(err);
                let _: AsyncSpawnResult = run_async_error_handle_function(
                    move |err_str| async move {
                        handle_error.call(err_str).await;
                    },
                    Arc::new(err_string),
                );
            }
        });
        self.sender.send(job_with_handler)
    }

    #[inline]
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
            let run_result: AsyncSpawnResult = run_async_function(move || async {
                job.call().await;
            });
            if let Err(err) = run_result {
                let err_string: String = tokio_error_to_string(err);
                let _: AsyncSpawnResult = run_async_error_handle_function(
                    move |err_str| async move {
                        handle_error.call(err_str).await;
                    },
                    Arc::new(err_string),
                );
            }
            let _: AsyncSpawnResult = run_async_function(move || async {
                finally.call().await;
            });
        });
        self.sender.send(job_with_handler)
    }
}
