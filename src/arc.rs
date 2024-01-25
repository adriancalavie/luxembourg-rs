use std::fmt;

#[derive(Debug, Clone)]
pub struct Arc {
    pub from_lat: f64,
    pub from_long: f64,
    pub to_lat: f64,
    pub to_long: f64,
    pub length: f64,
}

impl Arc {
    pub fn new(from_lat: f64, from_long: f64, to_lat: f64, to_long: f64, length: f64) -> Self {
        Self {
            from_lat,
            from_long,
            to_lat,
            to_long,
            length,
        }
    }
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[from_lat: {}][from_long: {}][to_lat: {}][to_long: {}][length: {}]",
            self.from_lat, self.from_long, self.to_lat, self.to_long, self.length
        )
    }
}
