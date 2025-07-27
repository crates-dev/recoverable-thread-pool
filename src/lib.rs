//! recoverable-thread-pool
//!
//! A thread pool that supports automatic recovery from panics,
//! allowing threads to restart after a panic. Useful for resilient
//! and fault-tolerant concurrency in network and web programming.

pub(crate) mod cfg;
pub(crate) mod thread_pool;
pub(crate) mod worker;

pub(crate) use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver, SendError, Sender},
};
pub(crate) use tokio::runtime::Builder;

pub use thread_pool::*;
pub use worker::*;
