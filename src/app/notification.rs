use notify_rust::{Notification, Timeout};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct NotificationLike {
  pub appname: String,
  pub summary: String,
  pub body: String,
  pub icon: String,

  #[cfg(target_os = "windows")]
  pub sound_name: Option<String>,

  #[cfg(target_os = "windows")]
  pub path_to_image: Option<String>,

  #[cfg(target_os = "windows")]
  pub app_id: Option<String>,

  pub timeout: TimeoutLike,
}

impl From<NotificationLike> for Notification {
  fn from(value: NotificationLike) -> Self {
    let mut notification = Notification::new();
    notification.appname = value.appname;
    notification.summary = value.summary;
    notification.body = value.body;
    notification.icon = value.icon;

    if let Some(v) = value.sound_name {
      notification.sound_name(&v);
    }
    if let Some(v) = value.path_to_image {
      notification.image_path(&v);
    }
    if let Some(v) = value.app_id {
      notification.app_id(&v);
    }

    dbg!(&notification);

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
