use std::fmt;

use egui::Pos2;

#[derive(Debug, Clone)]
pub struct Arc {
    pub from: Pos2,
    pub to: Pos2,
    pub length: f32,
}

impl Arc {
    pub fn new(from: Pos2, to: Pos2, length: f32) -> Self {
        Self { from, to, length }
    }
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[from: ({},{})][to: ({},{})][length: {}]",
            self.from.x, self.from.y, self.to.x, self.to.y, self.length
        )
    }
}
