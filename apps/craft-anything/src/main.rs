//! The Craft Anything desktop app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    craft_anything::run_with(|_| {})
}
