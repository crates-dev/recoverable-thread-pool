use std::thread::JoinHandle;

use lombok_macros::*;

#[allow(dead_code)]
#[derive(Lombok)]
pub struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}
