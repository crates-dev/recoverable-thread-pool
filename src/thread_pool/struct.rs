use crate::*;

/// A thread pool that can execute tasks concurrently.
///
/// Manages a collection of worker threads and provides methods
/// to submit tasks for execution.
pub struct ThreadPool {
    /// The collection of worker threads.
    #[allow(dead_code)]
    pub(super) workers: Vec<Worker>,
    /// The sender channel for submitting jobs to workers.
    pub(super) sender: Sender<ThreadPoolJob>,
}
