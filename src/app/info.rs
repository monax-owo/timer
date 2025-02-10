use iced::Task;

use crate::app::Message;

#[derive(Debug, Clone)]
pub enum Info {
  Send(String),
  Clear,
}

pub(crate) fn send<S: AsRef<str>>(info: S) -> Task<Message> {
  Task::done(Message::Info(Info::Send(info.as_ref().to_owned())))
}
