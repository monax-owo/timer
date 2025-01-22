mod app;
mod subscription;

use std::path::Path;

use app::App;
use iced::window;

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

  iced::daemon(APP_NAME, App::update, App::view)
    .theme(App::theme)
    .subscription(App::subscription)
    .scale_factor(|_, _| 1.4)
    .run_with(App::run)
}

fn load_tray_icon(path: &Path) -> tray_icon::Icon {
  let (icon_rgba, icon_width, icon_height) = load_image(path);
  tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn load_app_icon(path: &Path) -> window::Icon {
  let (icon_rgba, icon_width, icon_height) = load_image(path);
  window::icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn load_image(path: &Path) -> (Vec<u8>, u32, u32) {
  let image = image::open(path)
    .expect("Failed to open icon path")
    .into_rgba8();
  let (width, height) = image.dimensions();
  let rgba = image.into_raw();
  (rgba, width, height)
}
