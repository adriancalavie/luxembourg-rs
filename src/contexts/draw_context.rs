use egui::{Color32, Id, Pos2, Rect, Sense, Ui, Vec2};
use log::debug;

use crate::{
    models::{Edge, Node},
    translator::Translator,
    utils::constants::{DEFAULT_PAN, DEFAULT_ZOOM},
};

pub struct DrawingContext {
    pub clicked_node_id: Option<String>,
    pub zoom: f32,
    pub pan: Vec2,
    pub translator: Translator,
}

impl DrawingContext {
    pub fn new() -> Self {
        Self {
            clicked_node_id: None,
            zoom: DEFAULT_ZOOM,
            pan: DEFAULT_PAN,
            translator: Translator::default(),
        }
    }

    pub fn draw_node(&mut self, ui: &mut Ui, node: &Node, r: f32, color: Color32) {
        let position_on_screen = self.adjust_for_pan_and_zoom(&node.position);

        debug!("Drawing node {} at {:?}", node.id, position_on_screen);

        // Create an interactable area for the circle with a unique ID
        let node_response = ui.interact(
            Rect::from_center_size(position_on_screen, Vec2::splat(r * 2.0)),
            Id::new(node.id.clone()),
            Sense::click(),
        );

        ui.painter().circle_filled(position_on_screen, r, color);

        if node_response.clicked() {
            println!("Node {} was clicked", node.id);
            self.clicked_node_id = Some(node.id.clone());
        }
    }

    pub fn draw_edge(&mut self, edge: &Edge, ui: &mut Ui, color: Color32) {
        let from_position = self.adjust_for_pan_and_zoom(&edge.from);
        let to_position = self.adjust_for_pan_and_zoom(&edge.to);

        debug!("Drawing edge from {:?} to {:?}", from_position, to_position);

        ui.painter()
            .line_segment([from_position, to_position], (0.5, color));
    }

    fn adjust_for_pan_and_zoom(&self, position: &Pos2) -> Pos2 {
        (*position + self.pan) * self.zoom
    }
}
