//! gtk-compat: runtime GTK loader and small safe wrappers (Linux-only)
#![allow(dead_code)]
mod error;
mod loader;
mod symbols;
mod wrappers;
mod signals;

pub use error::Error;
pub use loader::{Loader, Version};
pub use wrappers::{Application, Button, Label, Window, BoxWidget, Orientation};
