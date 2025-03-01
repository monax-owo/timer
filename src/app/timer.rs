mod ticker;

use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use chrono::NaiveTime;
use ticker::{NormalTicker, Ticker};

pub struct Timer {
  ticker: Box<dyn Ticker>,
  data: Data,
}

impl Timer {
  pub fn tick(&mut self) -> bool {
    self.ticker.tick(&mut self.data)
  }

  pub fn set_ticker<T: Ticker + 'static>(&mut self, ticker: T) {
    self.ticker = Box::new(ticker);
  }

  pub fn list_tickers() -> Vec<Box<dyn Ticker>> {
    vec![Box::new(NormalTicker)]
  }
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      ticker: Box::new(NormalTicker),
      data: Data::default(),
    }
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
