use chrono::{format::StrftimeItems, Local};

use super::Data;

pub struct Ticker {
  pub name: &'static str,
  pub logic: fn(&mut Data) -> bool,
}

impl Ticker {
  const ALL_TICKER: &[Ticker] = &[wrap::<Normal>()];
}

impl Default for Ticker {
  fn default() -> Self {
    wrap::<Normal>()
  }
}

pub trait TickerBase {
  const NAME: &'static str;
  fn tick(timer: &mut Data) -> bool;
}

pub struct Normal;

impl TickerBase for Normal {
  const NAME: &'static str = "Normal";

  fn tick(data: &mut Data) -> bool {
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

const fn wrap<T: TickerBase>() -> Ticker {
  Ticker {
    name: T::NAME,
    logic: <Normal as TickerBase>::tick,
  }
}
