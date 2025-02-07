use std::{env::current_exe, time::Duration};

use configu::{Config, Configurable};
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
  pub check_rate: Duration,
  pub duration: Hms,
  pub notification: NotificationLike,
}

impl Default for UserConfig {
  fn default() -> Self {
    Self {
      check_rate: Duration::from_secs(3),
      duration: Hms::default(),
      notification: NotificationLike::default(),
    }
  }
}

// Hours, Minutes, Seconds
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Hms {
  hour: u8,
  min: u8,
  sec: u8,
}

impl Default for Hms {
  fn default() -> Self {
    Self {
      hour: 0,
      min: 30,
      sec: 0,
    }
  }
}

impl From<Hms> for Duration {
  fn from(value: Hms) -> Self {
    Self::from_secs(value.hour as u64 * 3600 + value.min as u64 * 60 + value.sec as u64)
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
  println!("{:#?}", config_file.to_string_lossy());
  let is_file = config_file.is_file();
  let mut config = Config::<T>::open(Some(config_file));

  if is_file {
    config.load()?;
  } else {
    *config = T::default();
  }

  Ok(config)
}

pub(crate) fn load(app: &mut super::App) {
  println!("config loaded");
  let config = &app.config;

  app.notification = config.notification.clone().into();
  app.timer.duration = config.duration.clone().into();
}
