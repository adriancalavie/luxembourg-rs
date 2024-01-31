use egui::{Pos2, Vec2};

use crate::{
    models::{Edge, Node},
    utils::constants::{DEFAULT_PAN, DEFAULT_ZOOM},
};

#[derive(Debug, Clone)]
pub struct DrawingContext {
    pub zoom: f32,
    pub pan: Vec2,
}

impl DrawingContext {
    pub fn new() -> Self {
        Self {
            zoom: DEFAULT_ZOOM,
            pan: DEFAULT_PAN,
        }
    }

    pub fn calc_node_coords(&self, node: &Node) -> Pos2 {
        self.adjust_for_pan_and_zoom(&node.position)
    }

    pub fn calc_edge_coords(&self, edge: &Edge) -> (Pos2, Pos2) {
        let from_position = self.adjust_for_pan_and_zoom(&edge.from.position);
        let to_position = self.adjust_for_pan_and_zoom(&edge.to.position);

        (from_position, to_position)
    }

    fn adjust_for_pan_and_zoom(&self, position: &Pos2) -> Pos2 {
        (*position + self.pan) * self.zoom
    }
}

#[cfg(test)]
mod tests {}
