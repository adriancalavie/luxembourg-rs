use std::{
    fmt,
    hash::{Hash, Hasher},
};

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

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
