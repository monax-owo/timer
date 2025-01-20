use std::time::Duration;

use chrono::{Local, NaiveTime};
use iced::{
  time,
  widget::{button, column, slider, text},
  window, Element, Subscription, Task, Theme,
};
use notify_rust::Notification;

const APP_NAME: &str = "Simple Timer";

fn main() -> iced::Result {
  iced::daemon(APP_NAME, update, view)
    .theme(theme)
    .subscription(subscription)
    .run_with(run)
}

#[derive(Debug)]
struct State {
  // app
  notification: Notification,
  check_rate: Duration,
  // timer
  duration: Duration,
  last: NaiveTime,
  next: NaiveTime,
}

#[derive(Debug, Clone)]
enum Message {
  WindowOpened(window::Id),
  Tick,
  ChangeCheckRate(u32),
  Notify,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
  match message {
    Message::WindowOpened(_id) => {}
    Message::Tick => {
      let now = Local::now().time();
      println!("now: {:#?}", now);
      println!("last: {:#?}", state.last);
      println!("next: {:#?}", state.next);

      if state.last + state.duration < now {
        state.last = now;
        state.next = now + state.duration;

        println!("elapsed!");
      }
    }
    Message::ChangeCheckRate(v) => state.check_rate = Duration::from_secs(v.into()),
    Message::Notify => state.notification.show().unwrap(),
  }
  Task::none()
}

fn view(state: &State, _id: window::Id) -> Element<Message> {
  let slider = slider(
    1..=60,
    state.check_rate.as_secs() as u32,
    Message::ChangeCheckRate,
  );
  column![
    text(state.duration.as_secs()),
    slider,
    button("notify").on_press(Message::Notify)
  ]
  .into()
}

fn theme(_state: &State, _window: window::Id) -> Theme {
  Theme::Dark
}

fn subscription(state: &State) -> Subscription<Message> {
  time::every(state.check_rate).map(|_| Message::Tick)
}

fn run() -> (State, Task<Message>) {
  // TODO:struct Timerに切り離す
  let now = Local::now().time();
  let duration = Duration::from_secs(10);
  let last = now;
  let next = now + duration;

  let state = State {
    notification: Notification::new()
      .appname(APP_NAME)
      .auto_icon()
      .summary("Test Summary")
      .body("Test Body")
      .finalize(),
    check_rate: Duration::from_secs(3),
    duration,
    last,
    next,
  };

  let (_id, open) = window::open(window::Settings::default());
  (state, open.map(Message::WindowOpened))
}
