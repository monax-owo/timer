use std::{env, time::Duration};

use chrono::{Local, NaiveTime};
use iced::{
  time,
  widget::{button, column, slider, text},
  window, Element, Subscription, Task, Theme,
};
use notify_rust::Notification;
use tray_icon::{TrayIcon, TrayIconBuilder};

const APP_NAME: &str = "Simple Timer";

fn main() -> iced::Result {
  #[cfg(debug_assertions)]
  if std::env::args().any(|arg| arg == "--gen-icons") {}
  {
    let assets_dir = std::env::current_dir().unwrap().join("assets");
    let input = assets_dir.join("icon.png");
    let output = assets_dir.join("icons");
    icon::command(icon::Options {
      input,
      output: Some(output),
      png: None,
      ios_color: "#000".to_string(),
    })
    .expect("failed to generate icons");
    println!("successfully generate icons");
  }

  iced::daemon(APP_NAME, App::update, App::view)
    .theme(App::theme)
    .subscription(App::subscription)
    .run_with(App::run)
}

struct App {
  // app
  tray_icon: TrayIcon,
  notification: Notification,
  check_rate: Duration,
  // timer
  timer: Timer,
}

#[derive(Debug)]
struct Timer {
  duration: Duration,
  last: NaiveTime,
  next: NaiveTime,
}

impl Default for Timer {
  fn default() -> Self {
    let now = Local::now().time();
    let duration = Duration::from_secs(10);
    let last = now;
    let next = now + duration;

    Self {
      duration,
      last,
      next,
    }
  }
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
        println!("last: {:#?}", self.timer.last);
        println!("next: {:#?}", self.timer.next);

        if self.timer.last + self.timer.duration < now {
          self.timer.last = now;
          self.timer.next = now + self.timer.duration;

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
    column![
      text(self.check_rate.as_secs()),
      slider(
        1..=60,
        self.check_rate.as_secs() as u32,
        Message::ChangeCheckRate,
      ),
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
    // tray icon
    let path = env::current_dir()
      .expect("failed")
      .join("assets/icons/32x32.png");
    let tray_icon = TrayIconBuilder::new()
      .with_icon(load_icon(&path))
      .with_title(APP_NAME)
      .with_tooltip(APP_NAME)
      .build()
      .expect("could not create tray icon");

    let state = App {
      tray_icon,
      notification: Notification::new()
        .appname(APP_NAME)
        .auto_icon()
        .summary("Test Summary")
        .body("Test Body")
        .finalize(),
      check_rate: Duration::from_secs(3),
      timer: Timer::default(),
    };

    let (_id, open) = window::open(window::Settings::default());
    (state, open.map(Message::WindowOpened))
  }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::open(path)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
