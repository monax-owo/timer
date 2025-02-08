use super::*;

pub(super) fn view(_app: &App) -> Element<Message> {
  { container(text("Config")).center(Fill) }.into()
}
