#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod subscription;

use app::App;
use iced::window;

// pub(crate) const APPID: &str = "com.squirrel.Discord.Discord";
pub(crate) const APPID: &str = "io.github.monax-owo.timer";
#[allow(unused)]
pub(crate) const UUID: &str = "b5d7e61b-dbcc-4f57-9aba-64908ce0111a";
pub(crate) const APP_NAME: &str = "Simple Timer";
pub(crate) const AUTO_START: bool = true;

fn main() -> iced::Result {
  #[cfg(debug_assertions)]
  if std::env::args().any(|arg| arg == "--gen-icons") {
    println!("generating icons...");
    let assets_dir = std::env::current_dir().unwrap().join("assets");
    let input = assets_dir.join("icon.png");
    let output = assets_dir.join("icons");
    icon::command(icon::Options {
      input,
      output: Some(output),
      png: None,
      ios_color: "#000".to_string(),
    })
    .expect("failed to generate icons");
    println!("successfully generate icons");
  }

  #[cfg(target_os = "windows")]
  (|| {
    #[cfg(debug_assertions)]
    if !std::env::args().any(|arg| arg == "--register") {
      return;
    }

    todo!("registration");
  })();

  iced::daemon(APP_NAME, App::update, App::view)
    .theme(App::theme)
    .subscription(App::subscription)
    .scale_factor(|_, _| 1.4)
    .settings(iced::Settings {
      id: Some(APPID.to_string()),
      ..Default::default()
    })
    .run_with(App::run)
}

fn load_tray_icon() -> tray_icon::Icon {
  const TRAY_ICON: &[u8] = include_bytes!("../assets/icons/32x32.png");

  let (rgba, width, height) = load_image(TRAY_ICON);

  tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to open icon")
}

fn load_app_icon() -> window::Icon {
  const APP_ICON: &[u8] = include_bytes!("../assets/icons/128x128.png");

  let (rgba, width, height) = load_image(APP_ICON);
  window::icon::from_rgba(rgba, width, height).expect("Failed to set app icon")
}

fn load_image(bytes: &[u8]) -> (Vec<u8>, u32, u32) {
  let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
    .expect("Failed to load image")
    .into_rgba8();
  let (width, height) = image.dimensions();
  let rgba = image.into_raw();
  (rgba, width, height)
}
