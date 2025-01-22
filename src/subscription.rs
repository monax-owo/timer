use iced::{
  futures::{SinkExt, Stream},
  stream,
};
use tray_icon::{
  menu::{MenuEvent, MenuId},
  TrayIconEvent,
};

#[derive(Debug, Clone)]
pub enum TrayEvent {
  MenuEvent(MenuId),
  IconEvent(TrayIconEvent),
}

pub fn tray_listener() -> impl Stream<Item = TrayEvent> {
  stream::channel(16, |mut output| async move {
    // let menu_event_receiver = MenuEvent::receiver();
    let icon_event_receiver = TrayIconEvent::receiver();

    loop {
      // if let Ok(MenuEvent { id }) = menu_event_receiver.recv() {
      //   output.send(TrayEvent::MenuEvent(id)).await.unwrap();
      // }

      if let Ok(e) = icon_event_receiver.recv() {
        output.send(TrayEvent::IconEvent(e)).await.unwrap();
      }
    }
  })
}
