use log::debug;

pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        debug!("screen width: {}, screen height: {}", width, height);
        Self { width, height }
    }
}

pub struct CoordsLimits {
    pub min_long: f64,
    pub max_long: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}

impl CoordsLimits {
    pub fn new(min_long: f64, max_long: f64, min_lat: f64, max_lat: f64) -> Self {
        debug!(
            "min_long: {}, max_long: {}, min_lat: {}, max_lat: {}",
            min_long, max_long, min_lat, max_lat
        );

        Self {
            min_long,
            max_long,
            min_lat,
            max_lat,
        }
    }
}