use iced::Task;

use crate::app::{Info, Message};

pub(crate) fn send<S: AsRef<str>>(info: S) -> Task<Message> {
  Task::done(Message::Info(Info::Send(info.as_ref().to_owned())))
}
