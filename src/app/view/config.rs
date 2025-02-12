use crate::app::config::ConfigEvent;

use super::*;

pub(super) fn view(_app: &App) -> Element<Message> {
  {
    Column::new()
      .push(pick_list(Theme::ALL, Some(&app.current_theme), Message::ChangeTheme).text_size(12))
      .push(button("Notify").on_press(Message::Notify))
      .push(
        Row::new()
          .push(button(text("Load")).on_press(Message::ConfigEvent(ConfigEvent::Load)))
          .push(button(text("Save")).on_press(Message::ConfigEvent(ConfigEvent::Save)))
          .align_y(Center)
          .spacing(4),
      )
      .width(Fill)
      .align_x(Center)
      .spacing(4)
  }
  .into()
}
