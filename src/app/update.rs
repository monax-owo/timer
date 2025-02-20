mod windows;

use std::time::Duration;

use iced::{
  window::{self, raw_window_handle::RawWindowHandle, settings::PlatformSpecific, Position},
  Task,
};
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent};

use super::{
  config::{load, save, ConfigEvent, Hms},
  info::{self, Info},
  App, Message,
};

pub(crate) fn update(app: &mut App, message: Message) -> Task<Message> {
  match message {
    Message::Tick => {
      if app.timer.tick() {
        println!("elapsed!");
        return Task::done(Message::Notify);
      }
    }
    Message::WindowEvent((e, id)) => match e {
      window::Event::Opened { .. } => {
        return Task::batch([
          window::change_icon(id, crate::util::icon::load_app_icon()),
          Task::done(Message::Tick),
        ])
      }
      window::Event::Closed => app.window = None,
      // TODO
      window::Event::Moved(point) => {
        app.window_pos = Some(point);
        dbg!(&app.window_pos);
      }
      window::Event::Unfocused => return window::close(id),
      _ => (),
    },
    Message::WindowCreateRequested => {
      if let Some(id) = app.window {
        return window::gain_focus(id);
      } else {
        // TODO:
        // app.task_tray.rect().unwrap()
        let mut settings = window::Settings {
          size: [600.0, 400.0].into(),
          resizable: false,
          transparent: true,
          platform_specific: PlatformSpecific {
            skip_taskbar: true,
            ..Default::default()
          },
          ..Default::default()
        };
        if let Some(v) = app.window_pos {
          settings.position = Position::Specific(v)
        }
        dbg!(&settings);

        let (id, open) = window::open(settings);
        app.window = Some(id);

        return open
          .chain(window::gain_focus(id))
          .discard()
          .chain(window::run_with_handle(id, move |handle| match handle.as_raw() {
            #[cfg(windows)]
            RawWindowHandle::Win32(handle) => windows::window_create_requested(handle),
            _ => (),
          }))
          .discard();
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
      ConfigEvent::Save => {
        save(app);
        return info::send("config saved");
      }
      ConfigEvent::Load => {
        load(app);
        return info::send("config loaded");
      }
    },
    Message::ChangeConfig(msg) => match msg {
      super::config::ChangeConfig::ChangeCheckRate(v) => app.config.check_rate = Hms::ZERO.second(v),
      super::config::ChangeConfig::ChangeDuration(duration) => {
        dbg!(duration);
      }
      super::config::ChangeConfig::ChangeTheme(theme) => app.current_theme = theme,
    },
    Message::ChangePage(page) => app.page = page,
    Message::Info(info) => match info {
      Info::Send(text) => {
        app.info = Some(text);

        return if let Some(handle) = &app.info_handle {
          handle.abort();

          app.info_handle = None;

          Task::none()
        } else {
          let (task, handle) = Task::future(async {
            tokio::time::sleep(Duration::from_secs(3)).await;
            Message::Info(Info::Clear)
          })
          .abortable();

          app.info_handle = Some(handle);

          task
        };
      }
      Info::Clear => app.info = None,
    },
    Message::Pause(v) => {
      app.timer.enable = !v;
      // if stopped
      if v {
        app.timer.next = None;
      }
      return Task::done(Message::Tick);
    }
    Message::Notify => app.notification.show().unwrap(),
    #[cfg(debug_assertions)]
    Message::ChangeDebugMode(v) => app.debug_mode = v,
  }
  Task::none()
}
