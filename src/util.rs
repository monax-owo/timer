pub(crate) mod icon {
  pub(crate) fn load_tray_icon() -> tray_icon::Icon {
    const TRAY_ICON: &[u8] = include_bytes!("../assets/icons/32x32.png");

    let (rgba, width, height) = load_image(TRAY_ICON);

    tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to open icon")
  }

  pub(crate) fn load_app_icon() -> iced::window::Icon {
    const APP_ICON: &[u8] = include_bytes!("../assets/icons/128x128.png");

    let (rgba, width, height) = load_image(APP_ICON);
    iced::window::icon::from_rgba(rgba, width, height).expect("Failed to set app icon")
  }

  pub(crate) fn load_image(bytes: &[u8]) -> (Vec<u8>, u32, u32) {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
      .expect("Failed to load image")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  }
}
