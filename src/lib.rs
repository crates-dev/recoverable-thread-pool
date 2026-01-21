//! recoverable-thread-pool
//!
//! A thread pool that supports automatic recovery from panics,
//! allowing threads to restart after a panic. Useful for resilient
//! and fault-tolerant concurrency in network and web programming.

pub(crate) mod thread_pool;
pub(crate) mod worker;

pub use {thread_pool::*, worker::*};

pub(crate) use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SendError, Sender},
    },
    thread::spawn,
};

pub(crate) use {recoverable_spawn::*, tokio::runtime::Builder};

#[cfg(test)]
pub(crate) use std::{thread::sleep, time::Duration};
