use notify_rust::{Notification, Timeout};
use serde::{Deserialize, Serialize};

use crate::{APPID, APP_NAME};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct NotificationLike {
  pub appname: String,
  pub summary: String,
  pub subtitle: Option<String>,
  pub body: String,
  pub icon: String,

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
      icon: String::default(),
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
    notification.icon = value.icon;

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
