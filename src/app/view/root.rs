use super::*;

pub fn view(app: &App, _id: window::Id) -> Element<Message> {
  let info = app.info.front().map(|info| text(info));

  {
    container(
      Column::new()
        .push(
          Row::new()
            .push_maybe(info)
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
