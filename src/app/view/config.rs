use crate::app::{
  config::{ChangeConfig, ConfigEvent},
  timer::ticker::Ticker,
};

use super::*;

pub(super) fn view(app: &App) -> Element<Message> {
  let mode_pick = pick_list(Ticker::ALL_TICKER, Some(app.current_ticker.clone()), |v| {
    Message::ChangeConfig(ChangeConfig::Ticker(v))
  })
  .text_size(12);

  let theme_pick = pick_list(Theme::ALL, Some(app.current_theme.clone()), |v| {
    Message::ChangeConfig(ChangeConfig::Theme(v))
  })
  .text_size(12);

  let mut config_items = vec![
    ("select mode", mode_pick.into()),
    ("select theme", theme_pick.into()),
    ("testing notify", config_button("Send").on_press(Message::Notify).into()),
  ];

  #[cfg(debug_assertions)]
  config_items.extend(vec![(
    "debug mode",
    config_button(app.debug_mode)
      .on_press(Message::ChangeDebugMode(!app.debug_mode))
      .into(),
  )]);

  let configs = create_config(config_items);

  Element::from({
    Column::new()
      .push(text("Configs").center())
      .push(scrollable(configs.spacing(8)).height(Fill))
      .push(
        container(
          Row::new()
            .push(config_button("Load").on_press(Message::ConfigEvent(ConfigEvent::Load)))
            .push(config_button("Save").on_press(Message::ConfigEvent(ConfigEvent::Save)))
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

fn config_button<'a, Msg: 'a>(text: impl text::IntoFragment<'a>) -> Button<'a, Msg> {
  Button::new(Text::new(text).center()).width(64)
}

fn config_item<'a, Msg: 'a>(
  label: impl Into<Element<'a, Msg>>,
  input: impl Into<Element<'a, Msg>>,
) -> Element<'a, Msg> {
  Element::from(Row::new().push(label).push(Space::with_width(Fill)).push(input))
}

fn create_config<'a, Msg: 'a>(items: Vec<(&'a str, Element<'a, Msg>)>) -> Column<'a, Msg> {
  items
    .into_iter()
    .map(|(label, element)| config_item(label, element))
    .fold(Column::new(), Column::push)
}
