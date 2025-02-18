use iced::window::raw_window_handle::Win32WindowHandle;
use windows::Win32::{
  Foundation::{BOOL, HWND, TRUE},
  Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED},
};

pub(super) fn window_create_requested(win32_window_handle: Win32WindowHandle) {
  let hwnd = HWND(win32_window_handle.hwnd.get() as *mut _);
  dbg!(hwnd);
  unsafe {
    DwmSetWindowAttribute(
      hwnd,
      DWMWA_TRANSITIONS_FORCEDISABLED,
      &TRUE as *const BOOL as *const std::ffi::c_void,
      std::mem::size_of::<BOOL>() as u32,
    )
    .unwrap()
  };
}
