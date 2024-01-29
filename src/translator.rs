use egui::{mutex::Mutex, Pos2};
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::utils::{constants::CANVAS_SIZE, WindowSize, HF64};

pub type TranslationArgs = (HF64, HF64); // longitude, latitude
pub type TranslationResults = Pos2; // x, y

pub static TRANSLATOR: Lazy<Mutex<Translator>> = Lazy::new(|| Mutex::new(Translator::default()));

#[derive(Debug, Clone)]
pub struct Translator {
    window_size: WindowSize,
    translation_cache: HashMap<TranslationArgs, TranslationResults>,
}

impl Translator {
    pub fn new(window_size: WindowSize) -> Self {
        Self {
            window_size,
            translation_cache: HashMap::new(),
        }
    }

    pub fn project(&mut self, longitude: f64, latitude: f64) -> Pos2 {
        let args = (HF64::new(longitude), HF64::new(latitude));

        if let Some(translation) = self.translation_cache.get(&args) {
            return *translation;
        }

        let position_on_screen = self.translate_coordinates(longitude, latitude);

        // memoisation
        self.translation_cache.insert(args, position_on_screen);

        position_on_screen
    }

    fn translate_coordinates(&self, longitude: f64, latitude: f64) -> Pos2 {
        let lon_in_radians = degrees_to_radians(longitude + 180.0);
        let lat_in_radians = degrees_to_radians(latitude);

        let radius = earth_radius(self.window_size.width as f64);
        let x = lon_in_radians * radius;

        let vertical_offset = radius
            * (std::f64::consts::PI / 4.0 + lat_in_radians / 2.0)
            * (std::f64::consts::PI / 4.0 + lat_in_radians / 2.0)
                .tan()
                .ln();

        let y = self.window_size.height as f64 / 2.0 - vertical_offset;

        Pos2::new(x as f32, y as f32)
    }
}

impl Default for Translator {
    fn default() -> Self {
        Translator::new(WindowSize::from(CANVAS_SIZE))
    }
}

fn earth_radius(circumference: f64) -> f64 {
    circumference / (2.0 * std::f64::consts::PI)
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
