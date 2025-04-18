use crate::*;
use recoverable_spawn::*;

pub type ThreadPoolJob = Box<dyn RecoverableFunction>;
pub type SendErrorBox = SendError<ThreadPoolJob>;
pub type SendResult = Result<(), SendErrorBox>;
