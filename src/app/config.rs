use std::env::current_exe;

use configu::{Config, Configurable};
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "timer.toml";

pub(crate) fn config<T: for<'de> Deserialize<'de> + Serialize + Default>() -> Result<Config<T>, configu::Error> {
  let config_file = current_exe()
    .expect("failed to get current exe")
    .parent()
    .expect("failed to get parent directory")
    .join(CONFIG_FILE);
  println!("{:#?}", config_file.to_string_lossy());
  let file_path = if config_file.is_file() { Some(config_file) } else { None };
  let mut config = Config::<T>::open(file_path);
  let res = config.load();

  if let Err(err) = res {
    match err {
      configu::Error::PathNotSpecified => (),
      _ => return Err(err),
    }
  }
  Ok(config)
}
