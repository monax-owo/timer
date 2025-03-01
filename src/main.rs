// TODO: https://github.com/rust-lang/rust/issues/67159#issuecomment-987882771
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod subscription;
mod util;

use app::App;
use clap::Parser;

pub(crate) const APPID: &str = "io.github.monax-owo.timer";
#[allow(unused)]
pub(crate) const UUID: &str = "b5d7e61b-dbcc-4f57-9aba-64908ce0111a";
pub(crate) const APP_NAME: &str = "Simple Timer";

#[derive(Parser, Debug)]
struct Args {
  #[cfg(debug_assertions)]
  #[arg(short, long)]
  gen_icons: bool,

  #[arg(short, long)]
  register: bool,
  #[arg(short, long)]
  un_register: bool,
}

fn main() -> iced::Result {
  let args = Args::parse();

  #[cfg(debug_assertions)]
  if args.gen_icons {
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

  init(args);

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

fn init(args: Args) {
  #[cfg(debug_assertions)]
  if !args.register {
    return;
  }

  #[cfg(windows)]
  {
    use windows::*;

    register();

    // TODO: temp dir
    // key.set_string("IconUri", temp_dir).unwrap();

    // TODO: unregister
    if false {
      get_app_user_model_id_key().remove_tree(APPID).unwrap();
    }
  }
}

#[cfg(windows)]
mod windows {
  use crate::{APPID, APP_NAME, UUID};
  use windows_registry::*;

  pub fn get_app_user_model_id_key() -> Key {
    CURRENT_USER.open(r"SOFTWARE\Classes\AppUserModelId").unwrap()
  }

  pub fn register() {
    let app_id = get_app_user_model_id_key().create(APPID).unwrap();

    let custom_activator = format!("{{{}}}", UUID.to_uppercase());
    app_id.set_string("CustomActivator", &custom_activator).unwrap();

    app_id.set_string("DisplayName", APP_NAME).unwrap();
  }
}
