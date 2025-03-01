mod normal;

use normal::Normal;

use super::Data;

pub struct Ticker {
  pub name: &'static str,
  pub logic: fn(&mut Data) -> bool,
}

impl Ticker {
  pub const ALL_TICKER: &[Ticker] = &[wrap::<Normal>()];
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

const fn wrap<T: TickerBase>() -> Ticker {
  Ticker {
    name: T::NAME,
    logic: <Normal as TickerBase>::tick,
  }
}
