#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod world;
mod furniture;
mod rendering;
mod xmltreewidget;