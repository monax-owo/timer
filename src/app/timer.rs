use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use chrono::{format::StrftimeItems, Local, NaiveTime};

pub struct Timer<T: Ticker> {
  ticker: T,
  pub data: Data,
}

impl<T: Ticker> Timer<T> {
  pub fn tick(&mut self) -> bool {
    self.ticker.tick(&mut self.data)
  }
}

impl<T: Ticker + Default> Default for Timer<T> {
  fn default() -> Self {
    Self {
      ticker: T::default(),
      data: Data::default(),
    }
  }
}

impl<T: Ticker> Deref for Timer<T> {
  type Target = Data;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<T: Ticker> DerefMut for Timer<T> {
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
