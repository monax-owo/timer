use std::time::Duration;

use iced::{
  window::{self, settings::PlatformSpecific},
  Task,
};
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent};

use super::{
  config::{load, ConfigEvent},
  App, Message,
};

pub(crate) fn update(app: &mut App, message: Message) -> Task<Message> {
  match message {
    Message::WindowEvent((e, id)) => match e {
      window::Event::Opened { .. } => {
        return Task::batch([
          window::change_icon(id, crate::util::icon::load_app_icon()),
          Task::done(Message::Tick),
        ])
      }
      window::Event::Closed => app.window = None,
      // TODO
      // window::Event::Focused => todo!(),
      // window::Event::Unfocused => todo!(),
      _ => (),
    },
    Message::WindowCreateRequested => {
      if let Some(id) = app.window {
        return Task::batch([
          window::minimize(id, false),
          window::change_mode(id, window::Mode::Windowed),
        ])
        .chain(window::gain_focus(id));
      } else {
        let (id, open) = window::open(window::Settings {
          size: [600.0, 400.0].into(),
          resizable: false,
          transparent: true,
          platform_specific: PlatformSpecific {
            skip_taskbar: true,
            ..Default::default()
          },
          ..Default::default()
        });
        app.window = Some(id);
        return open.chain(window::gain_focus(id)).map(|_| Message::Tick);
      }
    }
    Message::TrayMenuEvent(id) => match id.0.as_str() {
      App::SHOW_ID => return Task::done(Message::WindowCreateRequested),
      App::QUIT_ID => return iced::exit(),
      _ => (),
    },
    #[allow(clippy::single_match)]
    Message::TrayIconEvent(e) => {
      match e {
        TrayIconEvent::Click {
          button: MouseButton::Left,
          button_state: MouseButtonState::Up,
          ..
        } => return Task::done(Message::WindowCreateRequested),
        // TODO: right click
        _ => (),
      }
    }
    Message::ConfigEvent(e) => match e {
      ConfigEvent::Save => println!("config saved"),
      ConfigEvent::Load => {
        load(app);
      }
    },
    Message::Tick => {
      if app.timer.tick() {
        println!("elapsed!");
        return Task::done(Message::Notify);
      }
    }
    Message::ChangeCheckRate(v) => app.config.check_rate = Duration::from_secs(v.into()),
    Message::ChangeDuration(duration) => {
      dbg!(duration);
    }
    Message::ChangeTheme(theme) => app.current_theme = theme,
    Message::Pause(v) => {
      app.timer.enable = !v;
      // if stopped
      if v {
        app.timer.next = None;
      }
      return Task::done(Message::Tick);
    }
    Message::Notify => app.notification.show().unwrap(),
  }
  Task::none()
}
