use super::*;

pub fn view(app: &App, _id: window::Id) -> Element<Message> {
  let message = app.message.front().map(|msg| text(msg));

  {
    container(
      Column::new()
        .push(
          Row::new()
            .push_maybe(message)
            .push(Space::with_width(Fill))
            .push(
              button(text("!").align_x(Center))
                .on_press_with(|| {
                  Message::ChangePage(match app.page {
                    Page::Main => Page::Config,
                    Page::Config => Page::Main,
                  })
                })
                .height(32)
                .width(32),
            )
            .spacing(4),
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
