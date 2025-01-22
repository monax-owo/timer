use std::time::Duration;

use iced::{
  futures::{SinkExt, Stream},
  stream, Subscription,
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

pub fn tray_listener() -> Subscription<TrayEvent> {
  Subscription::batch([
    Subscription::run(menu_listener),
    Subscription::run(icon_listener),
  ])
}

fn menu_listener() -> impl Stream<Item = TrayEvent> {
  const CONNECTION: usize = 2;
  const TICK: u64 = 500;

  stream::channel(CONNECTION, |mut output| async move {
    let menu_event_receiver = MenuEvent::receiver();

    loop {
      if let Ok(MenuEvent { id }) = menu_event_receiver.recv() {
        output.send(TrayEvent::MenuEvent(id)).await.unwrap();
      }

      tokio::time::sleep(Duration::from_millis(TICK)).await;
    }
  })
}

fn icon_listener() -> impl Stream<Item = TrayEvent> {
  const CONNECTION: usize = 8;
  const TICK: u64 = 200;

  stream::channel(CONNECTION, |mut output| async move {
    let icon_event_receiver = TrayIconEvent::receiver();

    loop {
      if let Ok(e) = icon_event_receiver.recv() {
        output.send(TrayEvent::IconEvent(e)).await.unwrap();
      }

      tokio::time::sleep(Duration::from_millis(TICK)).await;
    }
  })
}
