use iced::{
  widget::{button, container, slider, text, Column, Row, Space},
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
        .push(
          container(
            button(text("!").align_x(Center))
              .height(Length::Fixed(32.0))
              .width(Length::Fixed(32.0)),
          )
          .align_x(End)
          .width(Length::Fill),
        )
        .push(
          container(
            Column::new()
              // TODO: button to square
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
              .spacing(4.0),
          )
          .align_y(Center)
          .height(Length::Fill),
        )
        .push(Space::new(Length::Fill, Length::Fixed(32.0)))
        .height(Length::Fill),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(8.0)
  }
  .into()
}
