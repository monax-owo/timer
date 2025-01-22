use std::{env, time::Duration};

use chrono::{Local, NaiveTime};
use iced::{
  alignment::{Horizontal, Vertical},
  padding, time,
  widget::{button, column, container, row, slider, text},
  window, Element, Length, Subscription, Task, Theme,
};
use notify_rust::Notification;
use tray_icon::{
  menu::{Menu, MenuId, MenuItem},
  TrayIcon, TrayIconBuilder, TrayIconEvent,
};

use crate::{subscription, APP_NAME, AUTO_START};

pub struct App {
  // app
  current_theme: Theme,
  #[allow(dead_code)]
  task_tray: TrayIcon,
  notification: Notification,
  check_rate: Duration,
  // timer
  timer: Timer,
}

#[derive(Debug)]
pub struct Timer {
  enable: bool,
  duration: Duration,
  last_next: Option<(NaiveTime, NaiveTime)>,
}

impl Timer {
  pub fn tick(&mut self) -> bool {
    if self.enable {
      let now = Local::now().time();
      let (last, next) = self.last_next.get_or_insert((now, now + self.duration));

      #[cfg(debug_assertions)]
      {
        println!("now: {:#?}", now);
        println!("last: {:#?}", last);
        println!("next: {:#?}", next);
      }

      let elapsed = *last + self.duration < now;

      if elapsed {
        self.last_next = Some((now, now + self.duration));
      }

      elapsed
    } else {
      false
    }
  }
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      enable: true,
      duration: Duration::from_secs(30),
      last_next: None,
    }
  }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
  WindowOpened(window::Id),
  TrayMenuEvent(MenuId),
  TrayIconEvent(TrayIconEvent),
  Tick,
  ChangeCheckRate(u32),
  // TODO
  ChangeTheme(Theme),
  // true = stop, false = start
  Pause(bool),
  Notify,
}

impl App {
  pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::WindowOpened(_id) => return Task::done(Message::Tick),
      Message::TrayMenuEvent(id) => println!("id: {:#?}", id),
      Message::TrayIconEvent(e) => println!("event: {:#?}", e),
      Message::Tick => {
        if self.timer.tick() {
          println!("elapsed!");
          return Task::done(Message::Notify);
        }
      }
      Message::ChangeCheckRate(v) => self.check_rate = Duration::from_secs(v.into()),
      Message::ChangeTheme(theme) => self.current_theme = theme,
      Message::Pause(v) => {
        self.timer.enable = !v;
        // if stopped
        if v {
          self.timer.last_next = None;
        }
        return Task::done(Message::Tick);
      }
      Message::Notify => self.notification.show().unwrap(),
    }
    Task::none()
  }

  // #[rustfmt::skip]
  pub(crate) fn view(&self, _id: window::Id) -> Element<Message> {
    #[rustfmt::skip]
    let check_rate_slider = slider(1..=60,self.check_rate.as_secs() as u32,Message::ChangeCheckRate);

    let next = match self.timer.last_next {
      Some((_last, next)) => next.format("%H:%M:%S").to_string(),
      None => "Break".to_string(),
    };

    let pause = if self.timer.enable { "Pause" } else { "Start" };

    {
      container(
        column![
          row![text("Next:"), text(next),],
          button(pause).on_press(Message::Pause(self.timer.enable)),
          row![text(self.check_rate.as_secs()), check_rate_slider],
          button("Notify").on_press(Message::Notify),
        ]
        .align_x(Horizontal::Center)
        .spacing(2.0),
      )
      .align_y(Vertical::Center)
      .height(Length::Fill)
      .width(Length::Fill)
      .padding(padding::all(8.0))
    }
    .into()
  }

  pub(crate) fn theme(&self, _window: window::Id) -> Theme {
    self.current_theme.clone()
  }

  pub(crate) fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
      time::every(self.check_rate).map(|_| Message::Tick),
      subscription::tray_listener().map(|e| match e {
        subscription::TrayEvent::MenuEvent(id) => Message::TrayMenuEvent(id),
        subscription::TrayEvent::IconEvent(e) => Message::TrayIconEvent(e),
      }),
    ])
  }

  pub(crate) fn run() -> (App, Task<Message>) {
    // tray icon
    let icons_dir = env::current_dir().expect("failed").join("assets/icons");
    let tray_icon = icons_dir.join("32x32.png");
    let app_icon = icons_dir.join("128x128.png");

    // task tray
    const SHOW_ID: &str = "show";
    const QUIT_ID: &str = "quit";

    let menu = Menu::new();
    menu
      .append_items(&[
        &MenuItem::with_id(SHOW_ID, "show", true, None),
        &MenuItem::with_id(QUIT_ID, "quit", true, None),
      ])
      .expect("failed to append tray items");

    let task_tray = TrayIconBuilder::new()
      .with_icon(crate::load_tray_icon(&tray_icon))
      .with_menu_on_left_click(false)
      .with_menu(Box::new(menu))
      .with_title(APP_NAME)
      .with_tooltip(APP_NAME)
      .build()
      .expect("could not create tray icon");
    // task tray

    // state
    let app_state = App {
      current_theme: Theme::Dark,
      task_tray,
      notification: Notification::new()
        .appname(APP_NAME)
        .auto_icon()
        .summary("Test Summary")
        .body("Test Body")
        .finalize(),
      check_rate: Duration::from_secs(3),
      timer: Timer {
        enable: AUTO_START,
        ..Default::default()
      },
    };
    // state

    let (_id, open) = window::open(window::Settings {
      size: [600.0, 400.0].into(),
      icon: Some(crate::load_app_icon(&app_icon)),
      ..Default::default()
    });
    (app_state, open.map(Message::WindowOpened))
  }
}
