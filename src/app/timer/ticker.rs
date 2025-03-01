use chrono::{format::StrftimeItems, Local};

use super::Data;

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
