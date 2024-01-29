#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod contexts;
mod map;
mod models;
mod parser;
mod translator;
mod utils;
mod components;

use dotenv::dotenv;
use eframe::egui;
use env_logger::{Builder, Target};
use map::Map;
use tokio::runtime::Runtime;

fn main() -> Result<(), eframe::Error> {
    if cfg!(debug) {
        std::env::set_var("RUST_LOG", "debug");
    }
    dotenv().ok();

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let rt = Runtime::new().expect("Unable to create Runtime");
    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1366.0, 900.0]),
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        // vsync: false,
        // renderer: eframe::Renderer::Wgpu,
        // wgpu_options: egui_wgpu::WgpuConfiguration {
        //     power_preference: eframe::wgpu::PowerPreference::HighPerformance,
        //     present_mode: eframe::wgpu::PresentMode::AutoNoVsync,
        //     supported_backends: eframe::wgpu::Backends::VULKAN,
        //     ..Default::default()
        // },
        ..Default::default()
    };
    eframe::run_native(
        "luxembourg-rs",
        options,
        Box::new(|_cc| Box::<Map>::from(Map::new())),
    )
}
