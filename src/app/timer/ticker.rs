mod normal;
mod pomodoro;

use normal::Normal;
use pomodoro::Pomodoro;

use super::Data;

pub type TickerState = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticker {
  pub name: &'static str,
  pub logic: fn(&mut Data, &mut u32) -> bool,
  pub state: TickerState,
}

impl Ticker {
  pub const ALL_TICKER: &[Ticker] = &[wrap::<Normal>(), wrap::<Pomodoro>()];
}

impl Default for Ticker {
  fn default() -> Self {
    wrap::<Normal>()
  }
}

impl ToString for Ticker {
  fn to_string(&self) -> String {
    self.name.to_owned()
  }
}

pub trait TickerBase {
  const NAME: &'static str;
  fn tick(timer: &mut Data, state: &mut u32) -> bool;
}

const fn wrap<T: TickerBase>() -> Ticker {
  Ticker {
    name: T::NAME,
    logic: T::tick,
    state: 0,
  }
}
