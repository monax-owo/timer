mod config;
mod main;
mod root;

pub use root::view;

use iced::{widget::*, window, Alignment::*, Element, Length::*};

use crate::app::{App, Message, Page};
