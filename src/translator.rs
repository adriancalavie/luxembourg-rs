use std::collections::HashMap;

use crate::{
    hashable_float::HF64,
    utils::{CoordsLimits, WindowSize},
};

pub type TranslationArgs = (HF64, HF64, HF64, HF64, HF64); // longitude, latitude, zoom, pan_x, pan_y
pub type TranslationResults = (f32, f32); // x, y

pub struct Translator {
    coords_limits: CoordsLimits,
    window_size: WindowSize,
    translation_cache: HashMap<TranslationArgs, TranslationResults>,
}

impl Translator {
    pub fn new(limits: CoordsLimits, window_size: WindowSize) -> Self {
        Self {
            coords_limits: limits,
            window_size,
            translation_cache: HashMap::new(),
        }
    }

    pub fn project(
        &mut self,
        longitude: f64,
        latitude: f64,
        zoom: f64,
        pan_x: f64,
        pan_y: f64,
    ) -> (f32, f32) {
        let args = (
            HF64::new(longitude),
            HF64::new(latitude),
            HF64::new(zoom),
            HF64::new(pan_x),
            HF64::new(pan_y),
        );

        if let Some(translation) = self.translation_cache.get(&args) {
            return *translation;
        }

        let (x, y) = self.translate_coordinates(longitude, latitude, zoom, pan_x, pan_y);

        // memoisation
        self.translation_cache.insert(args, (x, y));

        (x, y)
    }

    fn translate_coordinates(
        &self,
        longitude: f64,
        latitude: f64,
        zoom: f64,
        pan_x: f64,
        pan_y: f64,
    ) -> (f32, f32) {
        let lon_in_radians = degrees_to_radians(longitude + 180.0);
        let lat_in_radians = degrees_to_radians(latitude);

        let radius = earth_radius(self.window_size.width as f64);
        let x = lon_in_radians * radius + pan_x;

        let vertical_offset = radius
            * (std::f64::consts::PI / 4.0 + lat_in_radians / 2.0)
            * (std::f64::consts::PI / 4.0 + lat_in_radians / 2.0)
                .tan()
                .ln();

        let y = self.window_size.height as f64 / 2.0 - vertical_offset + pan_y;

        ((x * zoom) as f32, (y * zoom) as f32)
    }
}
impl Default for Translator {
    fn default() -> Self {
        const MIN_LAT: f64 = 5.734153;
        const MAX_LAT: f64 = 6.531256;
        const MIN_LONG: f64 = 50.182918;
        const MAX_LONG: f64 = 49.441140;

        Translator::new(
            CoordsLimits::new(MIN_LONG, MAX_LONG, MIN_LAT, MAX_LAT),
            WindowSize::new(1366, 900),
        )
    }
}

fn earth_radius(circumference: f64) -> f64 {
    circumference / (2.0 * std::f64::consts::PI)
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
