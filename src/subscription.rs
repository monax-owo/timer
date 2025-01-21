use iced::{
  futures::{SinkExt, Stream},
  stream,
};
use tray_icon::menu::{MenuEvent, MenuId};

pub fn tray_listener() -> impl Stream<Item = MenuId> {
  stream::channel(16, |mut output| async move {
    let (sender, mut reciever) = tokio::sync::mpsc::channel(16);

    std::thread::spawn(move || loop {
      if let Ok(event) = MenuEvent::receiver().recv() {
        sender.blocking_send(event).unwrap()
      }
    });

    loop {
      if let Some(MenuEvent { id }) = reciever.recv().await {
        output.send(id).await.unwrap();
      }
    }
  })
}
