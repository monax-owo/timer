use iced::{
  futures::{SinkExt, Stream},
  stream,
};
use tray_icon::menu::{MenuEvent, MenuId};

pub fn tray_menu_listener() -> impl Stream<Item = MenuId> {
  stream::channel(16, |mut output| async move {
    loop {
      if let Ok(MenuEvent { id }) = MenuEvent::receiver().recv() {
        println!("send");
        output.send(id).await.unwrap();
      }
    }
  })
}
