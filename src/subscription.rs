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
  stream::channel(4, |mut output| async move {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(4);

    // TODO:fix

    std::thread::spawn(move || loop {
      println!("0-1");
      if let Ok(MenuEvent { id }) = MenuEvent::receiver().recv() {
        println!("1");
        sender.blocking_send(TrayEvent::MenuEvent(id)).unwrap()
      }

      println!("0-2");
      if let Ok(e) = TrayIconEvent::receiver().recv() {
        println!("2");
        sender.blocking_send(TrayEvent::IconEvent(e)).unwrap()
      }
    });

    loop {
      if let Some(e) = receiver.recv().await {
        output.send(e).await.unwrap();
      }
    }
  })
}
