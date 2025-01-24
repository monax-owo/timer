use std::time::Duration;

use chrono::{format::StrftimeItems, Local, NaiveTime};

#[derive(Debug)]
pub struct Timer {
  pub enable: bool,
  pub duration: Duration,
  pub last_next: Option<(NaiveTime, NaiveTime)>,
}

impl Timer {
  pub fn tick(&mut self) -> bool {
    if self.enable {
      let now = Local::now().time();
      let (last, next) = self.last_next.get_or_insert((now, now + self.duration));

      #[cfg(debug_assertions)]
      {
        let fmt = StrftimeItems::new("%H:%M:%S");
        println!("now: {}", now.format_with_items(fmt.clone()));
        println!("last: {}", last.format_with_items(fmt.clone()));
        println!("next: {}", next.format_with_items(fmt.clone()));
      }

      let elapsed = *next < now;

      if elapsed {
        self.last_next = Some((now, now + self.duration));
        return true;
      }
    }
    false
  }
}

impl Default for Timer {
  fn default() -> Self {
    Self {
      enable: true,
      duration: Duration::from_secs(30),
      last_next: None,
    }
  }
}
