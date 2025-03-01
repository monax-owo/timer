use std::time::Duration;

use chrono::{format::StrftimeItems, Local};

use super::*;

pub struct Pomodoro;

impl TickerBase for Pomodoro {
  const NAME: &'static str = "Pomodoro";

  fn tick(data: &mut Data, state: &mut TickerState) -> bool {
    if data.enable {
      let is_break = is_break(state);

      let now = Local::now().naive_local();

      let duration = mode_to_min(is_break);
      let next = data.next.get_or_insert(now + duration);

      #[cfg(debug_assertions)]
      {
        let fmt = StrftimeItems::new("%H:%M:%S");
        println!("now: {}", now.format_with_items(fmt.clone()));
        println!("next: {}", next.format_with_items(fmt.clone()));

        dbg!(duration.as_secs());
      }

      let elapsed = &now > next;

      if elapsed {
        data.next = Some(now + mode_to_min(!is_break));

        *state = match is_break {
          true => 0,
          false => 1,
        };

        return true;
      }
    }
    false
  }
}

fn is_break(state: &mut u32) -> bool {
  match state {
    // default(25min)
    0 => false,
    // break(5min)
    1 => true,
    _ => {
      *state = 0;
      false
    }
  }
}

fn mode_to_min(is_break: bool) -> Duration {
  const MIN: u64 = 60;
  match is_break {
    false => Duration::from_secs(25 * MIN),
    true => Duration::from_secs(5 * MIN),
  }
}
