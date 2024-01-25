#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod arc;
mod hashable_float;
mod luxembourg_map;
mod node;
mod parser;
mod translator;
mod utils;

use dotenv::dotenv;
use eframe::egui;
use env_logger::{Builder, Target};
use luxembourg_map::LuxembourgMap;
use tokio::runtime::Runtime;

fn main() -> Result<(), eframe::Error> {
    dotenv().ok();

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let rt = Runtime::new().expect("Unable to create Runtime");
    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1366.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "luxembourg-rs",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<LuxembourgMap>::from(LuxembourgMap::new())
        }),
    )
}
