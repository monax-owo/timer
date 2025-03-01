use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use chrono::{format::StrftimeItems, Local, NaiveTime};

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
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      ticker: Box::new(NormalTicker::default()),
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

pub trait Ticker {
  fn tick(&self, timer: &mut Data) -> bool;
}

pub struct NormalTicker;

impl Default for NormalTicker {
  fn default() -> Self {
    Self
  }
}

impl Ticker for NormalTicker {
  fn tick(&self, data: &mut Data) -> bool {
    if data.enable {
      let now = Local::now().time();
      let next = data.next.get_or_insert(now + data.duration);

      #[cfg(debug_assertions)]
      {
        let fmt = StrftimeItems::new("%H:%M:%S");
        println!("now: {}", now.format_with_items(fmt.clone()));
        println!("next: {}", next.format_with_items(fmt.clone()));
      }

      let elapsed = dbg!(next.signed_duration_since(now).num_seconds()) <= 0;

      if elapsed {
        data.next = Some(*next + data.duration);
        return true;
      }
    }
    false
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
