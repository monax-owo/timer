mod config;
mod timer;
mod update;
mod view;

use std::time::Duration;

use configu::Config;
use iced::{event, time, window, Element, Event, Subscription, Task, Theme};
use notify_rust::Notification;
use tray_icon::{
  menu::{Menu, MenuId, MenuItem},
  TrayIcon, TrayIconBuilder, TrayIconEvent,
};

use crate::{app::config::ConfigEvent, subscription, APP_NAME, AUTO_START};

pub struct App {
  // app
  pub current_theme: Theme,
  #[allow(unused)]
  pub window: Option<window::Id>,
  #[allow(unused)]
  pub task_tray: TrayIcon,
  pub notification: Notification,
  // config
  #[allow(unused)]
  pub config: Config<config::UserConfig>,
  // timer
  pub timer: timer::Timer,
}

#[derive(Debug, Clone)]
pub enum Message {
  WindowEvent((window::Event, window::Id)),
  WindowCreateRequested,
  TrayMenuEvent(MenuId),
  TrayIconEvent(TrayIconEvent),
  ConfigEvent(ConfigEvent),
  Tick,
  ChangeCheckRate(u32),
  ChangeDuration(Duration),
  // TODO
  #[allow(unused)]
  ChangeTheme(Theme),
  // true = stop, false = start
  Pause(bool),
  Notify,
}

impl App {
  pub const SHOW_ID: &str = "show";
  pub const QUIT_ID: &str = "quit";

  pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
    update::update(self, message)
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
      time::every(self.config.check_rate).map(|_| Message::Tick),
      subscription::tray_listener().map(|e| match e {
        subscription::TrayEvent::MenuEvent(id) => Message::TrayMenuEvent(id),
        subscription::TrayEvent::IconEvent(e) => Message::TrayIconEvent(e),
      }),
    ])
  }

  pub(crate) fn run() -> (App, Task<Message>) {
    // config
    let config = config::config::<config::UserConfig>().expect("failed to initialize config");
    dbg!((*config).clone());
    // config

    // task tray
    let menu = Menu::new();
    menu
      .append_items(&[
        &MenuItem::with_id(Self::SHOW_ID, "show", true, None),
        &MenuItem::with_id(Self::QUIT_ID, "quit", true, None),
      ])
      .expect("failed to append tray items");

    let task_tray = TrayIconBuilder::new()
      .with_icon(crate::util::icon::load_tray_icon())
      .with_menu_on_left_click(false)
      .with_menu(Box::new(menu))
      .with_title(APP_NAME)
      .with_tooltip(APP_NAME)
      .build()
      .expect("could not create tray icon");
    // task tray

    // state
    let app_state = App {
      window: None,
      current_theme: Theme::Dark,
      task_tray,
      notification: Notification::default(),
      config,
    };
    // state

    (
      app_state,
      Task::batch([
        Task::done(Message::WindowCreateRequested),
        Task::done(Message::ConfigEvent(ConfigEvent::Load)),
      ]),
    )
  }
}
