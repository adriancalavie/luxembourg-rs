use std::fmt;

use egui::Pos2;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub position: Pos2,
}

impl Node {
    pub fn new(id: String, position: Pos2) -> Self {
        Self { id, position }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[id: {}][long: {}][lat: {}]",
            self.id, self.position.x, self.position.y
        )
    }
}
