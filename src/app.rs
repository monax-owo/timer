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

use crate::{app::config::ConfigEvent, subscription, APP_NAME};

pub struct App {
  // ui
  pub current_theme: Theme,
  pub page: Page,
  pub info: Option<String>,

  // app
  pub window: Option<window::Id>,
  pub task_tray: TrayIcon,
  pub notification: Notification,

  // config
  pub config: Config<config::UserConfig>,

  // timer
  pub timer: timer::Timer,
}

#[derive(Debug, Clone)]
pub enum Page {
  Main,
  Config,
}

#[derive(Debug, Clone)]
pub enum Message {
  Tick,

  // window
  WindowEvent((window::Event, window::Id)),
  WindowCreateRequested,

  // tray
  TrayMenuEvent(MenuId),
  TrayIconEvent(TrayIconEvent),

  // config
  ConfigEvent(ConfigEvent),

  ChangeCheckRate(u8),
  ChangeDuration(Duration),
  ChangeTheme(Theme),
  ChangePage(Page),
  // true = stop, false = start
  Pause(bool),
  Notify,
  Info(Info),
}

#[derive(Debug, Clone)]
pub enum Info {
  Send(String),
  Clear,
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
      time::every(self.config.check_rate.clone().into()).map(|_| Message::Tick),
      subscription::tray_listener().map(|e| match e {
        subscription::TrayEvent::MenuEvent(id) => Message::TrayMenuEvent(id),
        subscription::TrayEvent::IconEvent(e) => Message::TrayIconEvent(e),
      }),
    ])
  }

  pub(crate) fn run() -> (App, Task<Message>) {
    // config
    let config = config::config::<config::UserConfig>().expect("failed to initialize config");
    dbg!(&config);
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
    let notification = config.notification.clone().into();
    let duration = (&config.duration).into();

    let mut app_state = App {
      window: None,
      info: None,
      current_theme: Theme::Dark,
      page: Page::Main,
      task_tray,
      notification,
      config,
      timer: timer::Timer {
        duration,
        ..Default::default()
      },
    };

    config::load(&mut app_state);
    // state

    (app_state, Task::done(Message::WindowCreateRequested))
  }
}
