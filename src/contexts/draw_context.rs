use egui::{Color32, Id, Pos2, Rect, Sense, Ui, Vec2};
use log::debug;

use crate::{
    models::{Edge, Node},
    translator::Translator,
    utils::{
        constants::{DEFAULT_PAN, DEFAULT_ZOOM},
        errors::OutOfBoundsError,
        Assertion, WindowSize,
    },
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
        // clipping the node to the screen
        if let Ok(position_on_screen) = self.is_in_screen(&node.position, ui) {
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
    }

    pub fn draw_edge(&mut self, edge: &Edge, ui: &mut Ui, color: Color32) {
        let from_position = self.is_in_screen(&edge.from, ui);
        let to_position = self.is_in_screen(&edge.to, ui);

        if let (Ok(from_position), Ok(to_position)) = (from_position, to_position) {
            debug!("Drawing edge from {:?} to {:?}", from_position, to_position);
            ui.painter()
                .line_segment([from_position, to_position], (1.0, color));
        }
    }

    fn adjust_for_pan_and_zoom(&self, position: &Pos2) -> Pos2 {
        (*position + self.pan) * self.zoom
    }

    /// Returns the position adjusted for pan & zoom if it's inside the screen.
    ///
    /// This funcion needs to always be called on a position that's not yet adjusted for pan & zoom.
    ///
    /// This function will return an error if the position is outside the screen after adjusting for pan & zoom.
    fn is_in_screen(&self, position: &Pos2, ui: &mut Ui) -> Result<Pos2, OutOfBoundsError> {
        let screen_size = WindowSize::from(ui.ctx().screen_rect().size());

        let pos = self.adjust_for_pan_and_zoom(position);

        if position.x >= 0.0
            && position.x <= screen_size.width as f32
            && position.y >= 0.0
            && position.y <= screen_size.height as f32
        {
            Ok(pos)
        } else {
            Err(OutOfBoundsError::new(*position, screen_size))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
