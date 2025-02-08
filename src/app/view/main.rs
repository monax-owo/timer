use super::*;

pub(super) fn view(app: &App) -> Element<Message> {
  let check_rate_slider = slider(1..=60, app.config.check_rate.as_secs() as u32, Message::ChangeCheckRate);

  let next = match app.timer.next {
    Some(next) => format!("Next: {}", next.format("%H:%M:%S")),
    None => "Break".to_string(),
  };

  let pause = if app.timer.enable { "Pause" } else { "Start" };

  {
    Column::new()
      .push(text(next))
      .push(button(pause).on_press(Message::Pause(app.timer.enable)))
      .push(
        Row::new()
          .push(text(app.config.check_rate.as_secs()).center().width(32))
          .push(container(check_rate_slider.width(Fill)).padding([0, 12]))
          .push(Space::with_width(32))
          .align_y(Center)
          .padding([0, 8]),
      )
      .push(button("Notify").on_press(Message::Notify))
      .align_x(Center)
      .spacing(4)
  }
  .into()
}
