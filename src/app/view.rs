mod config;
mod main;

use iced::{widget::*, window, Alignment::*, Element, Length::*};

use crate::app::{App, Message, Page};

pub(crate) fn view(app: &App, _id: window::Id) -> Element<Message> {
  let page = match app.page {
    Page::Main => Page::Config,
    Page::Config => Page::Main,
  };

  {
    container(
      Column::new()
        .push(
          container(
            button(text("!").align_x(Center))
              .on_press(Message::ChangePage(page))
              .height(32)
              .width(32),
          )
          .align_right(Fill),
        )
        .push(
          container(match app.page {
            Page::Main => main::view(app),
            Page::Config => config::view(app),
          })
          .center_y(Fill),
        )
        .push(Space::new(Fill, 32))
        .height(Fill),
    )
    .height(Fill)
    .width(Fill)
    .padding(8)
  }
  .into()
}
