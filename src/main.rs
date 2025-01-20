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
  iced::daemon(APP_NAME, App::update, App::view)
    .theme(App::theme)
    .subscription(App::subscription)
    .run_with(App::run)
}

#[derive(Debug)]
struct App {
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

impl App {
  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::WindowOpened(_id) => {}
      Message::Tick => {
        let now = Local::now().time();
        println!("now: {:#?}", now);
        println!("last: {:#?}", self.last);
        println!("next: {:#?}", self.next);

        if self.last + self.duration < now {
          self.last = now;
          self.next = now + self.duration;

          println!("elapsed!");
          return Task::done(Message::Notify);
        }
      }
      Message::ChangeCheckRate(v) => self.check_rate = Duration::from_secs(v.into()),
      Message::Notify => self.notification.show().unwrap(),
    }
    Task::none()
  }

  fn view(&self, _id: window::Id) -> Element<Message> {
    let slider = slider(
      1..=60,
      self.check_rate.as_secs() as u32,
      Message::ChangeCheckRate,
    );
    column![
      text(self.duration.as_secs()),
      slider,
      button("notify").on_press(Message::Notify)
    ]
    .into()
  }

  fn theme(&self, _window: window::Id) -> Theme {
    Theme::Dark
  }

  fn subscription(&self) -> Subscription<Message> {
    time::every(self.check_rate).map(|_| Message::Tick)
  }

  fn run() -> (App, Task<Message>) {
    // TODO:struct Timerに切り離す
    let now = Local::now().time();
    let duration = Duration::from_secs(10);
    let last = now;
    let next = now + duration;

    let state = App {
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
}
