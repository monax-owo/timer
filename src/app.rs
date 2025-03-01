mod config;
mod info;
mod timer;
mod update;
mod view;

use config::ChangeConfig;
use configu::Config;
use iced::{event, time, window, Element, Event, Point, Subscription, Task, Theme};
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
  pub info_handle: Option<iced::task::Handle>,
  pub window_pos: Option<Point>,
  #[cfg(debug_assertions)]
  pub debug_mode: bool,

  // app
  pub window: Option<window::Id>,
  pub task_tray: TrayIcon,
  pub notification: Notification,

  // config
  pub config: Config<config::UserConfig>,

  // timer
  pub timer: timer::Timer,
  pub current_ticker: timer::ticker::Ticker,
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
  ChangeConfig(ChangeConfig),

  // ui
  ChangePage(Page),
  Info(info::Info),

  // true = stop, false = start
  Pause(bool),
  Notify,

  #[cfg(debug_assertions)]
  ChangeDebugMode(bool),
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

    let mut timer = timer::Timer::default();
    timer.duration = duration;

    let mut app_state = App {
      window: None,
      info: None,
      info_handle: None,
      window_pos: Some(Point::default()),
      #[cfg(debug_assertions)]
      debug_mode: true,
      current_theme: Theme::Dark,
      page: Page::Main,
      task_tray,
      notification,
      config,
      timer,
      current_ticker: timer::ticker::Ticker::default(),
    };

    config::load(&mut app_state);
    // state

    (app_state, Task::done(Message::WindowCreateRequested))
  }
}
