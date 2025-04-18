use crate::*;

pub struct ThreadPool {
    #[allow(dead_code)]
    pub(super) workers: Vec<Worker>,
    pub(super) sender: Sender<ThreadPoolJob>,
}
