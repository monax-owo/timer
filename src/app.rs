mod timer;
mod view;

use std::time::Duration;

use iced::{
  event, time,
  window::{self, settings::PlatformSpecific},
  Element, Event, Subscription, Task, Theme,
};
use notify_rust::Notification;
use tray_icon::{
  menu::{Menu, MenuId, MenuItem},
  TrayIcon, TrayIconBuilder, TrayIconEvent,
};

use crate::{subscription, APP_NAME, AUTO_START};

pub struct App {
  // app
  pub current_theme: Theme,
  #[allow(unused)]
  pub window: Option<window::Id>,
  #[allow(unused)]
  pub task_tray: TrayIcon,
  pub notification: Notification,
  pub check_rate: Duration,
  // timer
  pub timer: timer::Timer,
}

#[derive(Debug, Clone)]
pub enum Message {
  WindowEvent((window::Event, window::Id)),
  TrayMenuEvent(MenuId),
  TrayIconEvent(TrayIconEvent),
  Tick,
  ChangeCheckRate(u32),
  // TODO
  #[allow(unused)]
  ChangeTheme(Theme),
  // true = stop, false = start
  Pause(bool),
  Notify,
}

impl App {
  pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::WindowEvent((e, id)) => match e {
        window::Event::Opened { .. } => {
          return Task::batch([
            window::change_icon(id, crate::load_app_icon()),
            Task::done(Message::Tick),
          ])
        }
        window::Event::CloseRequested => (),
        // TODO
        // window::Event::Focused => todo!(),
        // window::Event::Unfocused => todo!(),
        _ => (),
      },
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

  pub(crate) fn view(&self, id: window::Id) -> Element<Message> {
    view::view(self, id)
  }

  pub(crate) fn theme(&self, _window: window::Id) -> Theme {
    self.current_theme.clone()
  }

  pub(crate) fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
      event::listen_with(|e, _status, id| match e {
        Event::Window(e) => Some(Message::WindowEvent((e, id))),
        _ => None,
      }),
      time::every(self.check_rate).map(|_| Message::Tick),
      subscription::tray_listener().map(|e| match e {
        subscription::TrayEvent::MenuEvent(id) => Message::TrayMenuEvent(id),
        subscription::TrayEvent::IconEvent(e) => Message::TrayIconEvent(e),
      }),
    ])
  }

  pub(crate) fn run() -> (App, Task<Message>) {
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
      .with_icon(crate::load_tray_icon())
      .with_menu_on_left_click(false)
      .with_menu(Box::new(menu))
      .with_title(APP_NAME)
      .with_tooltip(APP_NAME)
      .build()
      .expect("could not create tray icon");
    // task tray

    // window
    let (id, _open) = window::open(window::Settings {
      size: [600.0, 400.0].into(),
      platform_specific: PlatformSpecific {
        skip_taskbar: true,
        ..Default::default()
      },
      exit_on_close_request: false,
      ..Default::default()
    });
    // window

    // state
    let app_state = App {
      window: Some(id),
      current_theme: Theme::Dark,
      task_tray,
      notification: Notification::new()
        .appname(APP_NAME)
        .auto_icon()
        .summary("Test Summary")
        .body("Test Body")
        .finalize(),
      check_rate: Duration::from_secs(3),
      timer: timer::Timer {
        enable: AUTO_START,
        ..Default::default()
      },
    };
    // state

    (app_state, Task::none())
  }
}
