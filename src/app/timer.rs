pub mod ticker;

use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use chrono::NaiveTime;
use ticker::Ticker;

#[derive(Default)]
pub struct Timer {
  ticker: Ticker,
  data: Data,
}

impl Timer {
  pub fn tick(&mut self) -> bool {
    (self.ticker.logic)(&mut self.data, &mut self.ticker.state)
  }
}

impl Deref for Timer {
  type Target = Data;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for Timer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

#[derive(Debug)]
pub struct Data {
  pub enable: bool,
  pub duration: Duration,
  pub next: Option<NaiveTime>,
}

impl Default for Data {
  fn default() -> Self {
    Self {
      enable: true,
      duration: Duration::from_secs(60 * 30),
      next: None,
    }
  }
}
