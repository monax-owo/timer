use super::*;

pub(super) fn view(_app: &App) -> Element<Message> {
  { container(text("Config")).height(Fill).width(Fill) }.into()
}
