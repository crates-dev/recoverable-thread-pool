use crate::worker::r#type::Worker;
use std::sync::mpsc::Sender;

pub type ThreadPoolJob = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    #[allow(dead_code)]
    pub(super) workers: Vec<Worker>,
    pub(super) sender: Sender<ThreadPoolJob>,
}
