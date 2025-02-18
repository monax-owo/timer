use std::{env::current_exe, time::Duration};

use configu::{Config, Configurable};
use iced::Theme;
use notify_rust::{Notification, Timeout};
use serde::{Deserialize, Serialize};

use crate::{APPID, APP_NAME};

const CONFIG_FILE: &str = "timer.toml";

#[derive(Debug, Clone)]
pub enum ConfigEvent {
  Save,
  Load,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct UserConfig {
  #[serde(with = "theme")]
  pub theme: Theme,
  pub check_rate: Hms,
  pub duration: Hms,
  pub notification: NotificationLike,
}

impl Default for UserConfig {
  fn default() -> Self {
    Self {
      theme: Theme::Dark,
      check_rate: Hms::ZERO.second(3),
      duration: Hms::default(),
      notification: NotificationLike::default(),
    }
  }
}

mod theme {
  use iced::Theme;
  use serde::{Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(t: &Theme, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    s.serialize_str(&t.to_string())
  }

  pub fn deserialize<'de, D>(d: D) -> Result<Theme, D::Error>
  where
    D: Deserializer<'de>,
  {
    fn from_str(str: &str) -> Option<Theme> {
      Theme::ALL
        .iter()
        .find(|v| v.to_string().trim().replace(' ', "") == str)
        .cloned()
    }

    let s = String::deserialize(d)?;
    Ok(from_str(&s).unwrap())
  }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hms {
  pub hour: u8,
  pub minute: u8,
  pub second: u8,
}

impl Hms {
  pub const ZERO: Self = Self {
    hour: 0,
    minute: 0,
    second: 0,
  };

  pub const MAX: Self = Self {
    hour: u8::MAX,
    minute: u8::MAX,
    second: u8::MAX,
  };

  pub fn new(hour: u8, minute: u8, second: u8) -> Self {
    Self { hour, minute, second }
  }

  pub fn second(mut self, second: u8) -> Self {
    self.second = second;
    self
  }

  pub fn minute(mut self, minute: u8) -> Self {
    self.minute = minute;
    self
  }

  pub fn hour(mut self, hour: u8) -> Self {
    self.hour = hour;
    self
  }
}

impl Default for Hms {
  fn default() -> Self {
    Self::ZERO.minute(30)
  }
}

impl From<Hms> for Duration {
  fn from(value: Hms) -> Self {
    Self::from(&value)
  }
}

impl From<&Hms> for Duration {
  fn from(value: &Hms) -> Self {
    Self::from_secs(value.hour as u64 * 3600 + value.minute as u64 * 60 + value.second as u64)
  }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct NotificationLike {
  pub appname: String,
  pub summary: String,
  pub subtitle: Option<String>,
  pub body: String,
  // pub icon: String,
  #[cfg(target_os = "windows")]
  pub sound_name: Option<String>,

  #[cfg(target_os = "windows")]
  pub path_to_image: Option<String>,

  #[cfg(target_os = "windows")]
  pub app_id: String,

  pub timeout: TimeoutLike,
}

impl Default for NotificationLike {
  fn default() -> Self {
    Self {
      appname: APP_NAME.to_owned(),
      summary: "Elapsed now".to_owned(),
      subtitle: None,
      body: String::default(),
      sound_name: None,
      path_to_image: None,
      app_id: APPID.to_owned(),
      timeout: TimeoutLike::default(),
    }
  }
}

impl From<NotificationLike> for Notification {
  fn from(value: NotificationLike) -> Self {
    let mut notification = Notification::new();
    notification.appname = value.appname;
    notification.summary = value.summary;
    notification.subtitle = value.subtitle;
    notification.body = value.body;

    if let Some(v) = value.sound_name {
      notification.sound_name(&v);
    }
    if let Some(v) = value.path_to_image {
      notification.image_path(&v);
    }

    notification.app_id(&value.app_id);

    notification.timeout(value.timeout);

    notification.finalize()
  }
}

#[derive(Deserialize, Serialize, Debug, Default, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutLike {
  #[default]
  Default,
  Never,
  Milliseconds(u32),
}

impl From<TimeoutLike> for Timeout {
  fn from(value: TimeoutLike) -> Self {
    match value {
      TimeoutLike::Default => Timeout::Default,
      TimeoutLike::Never => Timeout::Never,
      TimeoutLike::Milliseconds(ms) => Timeout::Milliseconds(ms),
    }
  }
}

pub(crate) fn config<T: for<'de> Deserialize<'de> + Serialize + Default>() -> Result<Config<T>, configu::Error> {
  let config_file = current_exe()
    .expect("failed to get current exe")
    .parent()
    .expect("failed to get parent directory")
    .join(CONFIG_FILE);

  let is_file = config_file.is_file();
  let mut config = Config::<T>::open(Some(config_file));

  if is_file {
    config.load()?;
  } else {
    config.file_path = None;
    *config = T::default();
  }

  Ok(config)
}

pub(crate) fn load(app: &mut super::App) {
  dbg!(&app.config.file_path);
  app.config.load().or_else(uncheck_path_not_specified).unwrap();

  app.notification = app.config.notification.clone().into();
  app.timer.duration = (&app.config.duration).into();

  println!("config loaded");
}

pub(crate) fn save(app: &super::App) {
  app.config.save().or_else(uncheck_path_not_specified).unwrap();

  println!("config saved");
}

fn uncheck_path_not_specified(err: configu::Error) -> Result<(), configu::Error> {
  match err {
    configu::Error::PathNotSpecified => Ok(()),
    _ => Err(err),
  }
}
