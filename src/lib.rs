#![warn(clippy::all, rust_2018_idioms)]
mod app;
mod pane_manager;
mod panes;
mod nodes;

pub use app::App;
pub use pane_manager::PaneManager;