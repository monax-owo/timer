use crate::app::config::ConfigEvent;

use super::*;

pub(super) fn view(app: &App) -> Element<Message> {
  let configs = Column::new()
    .push(pick_list(Theme::ALL, Some(&app.current_theme), Message::ChangeTheme).text_size(12))
    .push(button("Notify").on_press(Message::Notify));

  {
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
  }
  .into()
}
