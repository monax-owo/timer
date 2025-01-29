use iced::{
  widget::{button, container, slider, text, Column, Row},
  window,
  Alignment::*,
  Element, Length,
};

use crate::app::{App, Message};

pub(crate) fn view(app: &App, _id: window::Id) -> Element<Message> {
  let check_rate_slider = slider(1..=60, app.config.check_rate.as_secs() as u32, Message::ChangeCheckRate);

  let next = match app.timer.next {
    Some(next) => format!("Next: {}", next.format("%H:%M:%S")),
    None => "Break".to_string(),
  };

  let pause = if app.timer.enable { "Pause" } else { "Start" };

  {
    container(
      Column::new()
        // TODO: button to square
        .push(container(button("!")).align_x(End).width(Length::Fill))
        .push(text(next))
        .push(button(pause).on_press(Message::Pause(app.timer.enable)))
        .push(
          Row::new()
            .push(text(app.config.check_rate.as_secs()))
            .push(container(check_rate_slider.width(Length::Fill)).padding([0, 12]))
            .align_y(Center)
            .padding([0, 8]),
        )
        .push(button("Notify").on_press(Message::Notify))
        .align_x(Center)
        .spacing(2.0),
    )
    .align_x(Center)
    .align_y(Center)
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(8.0)
  }
  .into()
}
