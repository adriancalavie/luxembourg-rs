use std::fmt;

use egui::Pos2;

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: Pos2,
    pub to: Pos2,
    pub length: f32,
}

impl Edge {
    pub fn new(from: Pos2, to: Pos2, length: f32) -> Self {
        Self { from, to, length }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[from: ({},{})][to: ({},{})][length: {}]",
            self.from.x, self.from.y, self.to.x, self.to.y, self.length
        )
    }
}
