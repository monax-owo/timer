use crate::app::config::ConfigEvent;

use super::*;

pub(super) fn view(app: &App) -> Element<Message> {
  let theme_pick = pick_list(Theme::ALL, Some(&app.current_theme), Message::ChangeTheme).text_size(12);

  let configs = create_config(vec![
    ("select theme", theme_pick.into()),
    ("testing notify", button("Send").on_press(Message::Notify).into()),
  ]);

  Element::from({
    Column::new()
      .push(text("Configs").center())
      .push(scrollable(configs.spacing(8)).height(Fill))
      .push(
        container(
          Row::new()
            .push(button(text("Load")).on_press(Message::ConfigEvent(ConfigEvent::Load)))
            .push(button(text("Save")).on_press(Message::ConfigEvent(ConfigEvent::Save)))
            .align_y(Center)
            .spacing(4),
        )
        .align_right(Fill),
      )
      .width(Fill)
      .align_x(Center)
      .spacing(12)
  })
}

fn config_item<'a, Msg: 'a>(
  label: impl Into<Element<'a, Msg>>,
  input: impl Into<Element<'a, Msg>>,
) -> Element<'a, Msg> {
  Element::from(Row::new().push(label).push(Space::with_width(Fill)).push(input))
}

fn create_config<'a, Msg: 'a>(items: Vec<(&'a str, Element<'a, Msg>)>) -> Column<'a, Msg> {
  let vec = items
    .into_iter()
    .map(|(label, element)| config_item(label, element))
    .collect::<Vec<_>>();

  Column::new().extend(vec.into_iter())
}
