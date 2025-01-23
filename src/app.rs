use std::time::Duration;

use chrono::{format::StrftimeItems, Local, NaiveTime};
use iced::{
  time,
  widget::{button, column, container, row, slider, text},
  window::{self, settings::PlatformSpecific},
  Alignment::Center,
  Element, Length, Subscription, Task, Theme,
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
  pub task_tray: TrayIcon,
  pub notification: Notification,
  pub check_rate: Duration,
  // timer
  pub timer: Timer,
}

#[derive(Debug)]
pub struct Timer {
  pub enable: bool,
  pub duration: Duration,
  pub last_next: Option<(NaiveTime, NaiveTime)>,
}

impl Timer {
  pub fn tick(&mut self) -> bool {
    if self.enable {
      let now = Local::now().time();
      let (last, next) = self.last_next.get_or_insert((now, now + self.duration));

      #[cfg(debug_assertions)]
      {
        let fmt = StrftimeItems::new("%H:%M:%S");
        println!("now: {}", now.format_with_items(fmt.clone()));
        println!("last: {}", last.format_with_items(fmt.clone()));
        println!("next: {}", next.format_with_items(fmt.clone()));
      }

      let elapsed = *next < now;

      if elapsed {
        self.last_next = Some((now, now + self.duration));
        return true;
      }
    }
    false
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
pub enum Message {
  WindowOpened(window::Id),
  WindowCloseRequested(window::Id),
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
      Message::WindowOpened(id) => {
        return Task::batch([crate::set_app_icon(id), Task::done(Message::Tick)])
      }
      Message::WindowCloseRequested(_id) => (),
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
          row![
            text(self.check_rate.as_secs()),
            container(check_rate_slider.width(Length::Fill)).padding([0, 12])
          ]
          .align_y(Center)
          .padding([0, 8]),
          button("Notify").on_press(Message::Notify),
        ]
        .align_x(Center)
        .spacing(2.0),
      )
      .align_x(Center)
      .align_y(Center)
      .height(Length::Fill)
      .width(Length::Fill)
      .padding(8.0)
    }
    .into()
  }

  pub(crate) fn theme(&self, _window: window::Id) -> Theme {
    self.current_theme.clone()
  }

  pub(crate) fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
      window::close_requests().map(Message::WindowCloseRequested),
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
      platform_specific: PlatformSpecific {
        skip_taskbar: true,
        ..Default::default()
      },
      exit_on_close_request: false,
      ..Default::default()
    });

    (app_state, open.map(Message::WindowOpened))
  }
}
