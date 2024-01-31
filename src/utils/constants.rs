use egui::{Pos2, Vec2};

pub const CANVAS_SIZE: Pos2 = Pos2::new(1366., 900.);

pub const MIN_PAN: Vec2 = Vec2::new(-1000., -1000.);
pub const MAX_PAN: Vec2 = Vec2::new(1000., 1000.);
pub const MAX_ZOOM: f32 = 1000.0;

pub const DEFAULT_ZOOM: f32 = 100.0;
pub const DEFAULT_PAN: Vec2 = Vec2::new(-700., -179.);

// these are the bounds of the Luxembourg
// they are here only for historical reasons
pub const _LUX_MIN_LAT: f64 = 5.734153;
pub const _LUX_MAX_LAT: f64 = 6.531256;
pub const _LUX_MIN_LONG: f64 = 50.182918;
pub const _LUX_MAX_LONG: f64 = 49.441140;

pub mod xml_data {
    pub const MAP2_XML: &[u8] = include_bytes!("../../res/map2.xml");
    pub const TEST_XML: &[u8] = include_bytes!("../../res/test.xml");
}
