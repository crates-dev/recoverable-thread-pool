use crate::worker::r#type::Worker;
use recoverable_spawn::*;
use std::sync::mpsc::{SendError, Sender};

pub type ThreadPoolJob = Box<dyn RecoverableFunction>;
pub type SendErrorBox = SendError<ThreadPoolJob>;
pub type SendResult = Result<(), SendErrorBox>;

pub struct ThreadPool {
    #[allow(dead_code)]
    pub(super) workers: Vec<Worker>,
    pub(super) sender: Sender<ThreadPoolJob>,
}
