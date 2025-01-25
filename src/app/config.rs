use std::env::current_exe;

use configu::Config;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "timer.toml";

pub(crate) fn config<T: for<'de> Deserialize<'de> + Serialize + Default>() -> Config<T> {
  let config_file = current_exe().expect("failed to get current exe").join(CONFIG_FILE);
  let file_path = if config_file.is_file() { Some(config_file) } else { None };
  Config::<T>::open(file_path)
}
