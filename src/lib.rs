pub mod config;
pub mod gui;
pub mod sorting;

#[cfg(not(target_arch = "wasm32"))]
pub mod tui;
