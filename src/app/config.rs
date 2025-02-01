use std::env::current_exe;

use configu::{Config, Configurable};
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "timer.toml";

#[derive(Debug, Clone)]
pub enum ConfigEvent {
  Save,
  Load,
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
