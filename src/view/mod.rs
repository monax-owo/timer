use iced::{
  widget::{button, column, container, row, slider, text},
  window::{self},
  Alignment::Center,
  Element, Length,
};

use crate::app::{App, Message};

pub(crate) fn view(app: &App, _id: window::Id) -> Element<Message> {
  let check_rate_slider = slider(
    1..=60,
    app.check_rate.as_secs() as u32,
    Message::ChangeCheckRate,
  );

  let next = match app.timer.last_next {
    Some((_last, next)) => next.format("%H:%M:%S").to_string(),
    None => "Break".to_string(),
  };

  let pause = if app.timer.enable { "Pause" } else { "Start" };

  {
    container(
      column![
        row![text("Next:"), text(next),],
        button(pause).on_press(Message::Pause(app.timer.enable)),
        row![
          text(app.check_rate.as_secs()),
          container(check_rate_slider.width(Length::Fill)).padding([0, 12])
        ]
        .align_y(Center)
        .padding([0, 8]),
        button("Notify").on_press(Message::Notify),
      ]
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
