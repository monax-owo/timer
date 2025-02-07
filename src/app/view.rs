mod config;
mod main;

use iced::{widget::*, window, Alignment::*, Element, Length::*};

use crate::app::{App, Message, Page};

pub(crate) fn view(app: &App, _id: window::Id) -> Element<Message> {
  {
    container(
      Column::new()
        .push(
          container(button(text("!").align_x(Center)).height(Fixed(32.0)).width(Fixed(32.0)))
            .align_x(End)
            .width(Fill),
        )
        .push(
          container(match app.page {
            Page::Main => main::view(app),
            Page::Config => config::view(app),
          })
          .align_y(Center)
          .height(Fill),
        )
        .push(Space::new(Fill, Fixed(32.0)))
        .height(Fill),
    )
    .height(Fill)
    .width(Fill)
    .padding(8.0)
  }
  .into()
}
