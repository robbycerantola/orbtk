#![crate_name="orbtk"]
#![crate_type="lib"]
#![deny(warnings)]
#![feature(const_fn)]

extern crate orbclient;
extern crate orbimage;
extern crate cssparser;
#[macro_use]
extern crate lazy_static;

pub use orbclient::color::Color;
pub use orbclient::renderer::Renderer;

pub use cell::CloneCell;
pub use dialogs::*;
pub use event::Event;
pub use point::Point;
pub use rect::Rect;
pub use traits::*;
pub use widgets::*;
pub use window::{InnerWindow, Window, WindowBuilder};

pub mod cell;
pub mod dialogs;
pub mod event;
pub mod point;
pub mod rect;
pub mod traits;
pub mod widgets;
pub mod window;
pub mod draw;
pub mod theme;
