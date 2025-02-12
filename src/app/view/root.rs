use iced::Color;

use super::*;

pub fn view(app: &App, _id: window::Id) -> Element<Message> {
  let info = app.info.as_ref().map(text);

  let view = Element::from({
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
  });

  #[cfg(debug_assertions)]
  if app.explain {
    return view.explain(Color::from_rgba(0.0, 1.0, 1.0, 0.2));
  }

  view
}
