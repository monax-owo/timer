use super::*;

pub(super) fn view(app: &App) -> Element<Message> {
  let next = match app.timer.next {
    Some(next) => format!("Next: {}", next.format("%H:%M:%S")),
    None => "Break".to_string(),
  };

  let pause = if app.timer.enable { "Pause" } else { "Start" };

  Element::from({
    Column::new()
      .push(text(next).size(20))
      .push(button(pause).on_press(Message::Pause(app.timer.enable)))
      .width(Fill)
      .align_x(Center)
      .spacing(12)
  })
}
