use chrono::{format::StrftimeItems, Local};

use super::*;

pub struct Normal;

impl TickerBase for Normal {
  const NAME: &'static str = "Normal";

  fn tick(data: &mut Data, _state: &mut TickerState) -> bool {
    if data.enable {
      let now = Local::now().naive_local();
      let next = data.next.get_or_insert(now + data.duration);

      #[cfg(debug_assertions)]
      {
        let fmt = StrftimeItems::new("%H:%M:%S");
        println!("now: {}", now.format_with_items(fmt.clone()));
        println!("next: {}", next.format_with_items(fmt.clone()));
      }

      let elapsed = &now > next;

      if elapsed {
        data.next = Some(*next + data.duration);
        return true;
      }
    }
    false
  }
}
