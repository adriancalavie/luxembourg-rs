use egui::{Color32, Id, Pos2, Rect, Sense, Ui, Vec2};

use crate::{
    arc::Arc,
    node::Node,
    parser::parse_xml,
    translator::{TranslationResults, Translator},
};

pub struct LuxembourgMap {
    nodes: Vec<Node>,
    arcs: Vec<Arc>,
    mouse_pos: Pos2,
    draw_ctx: DrawingContext,
}

impl LuxembourgMap {
    pub fn new() -> Self {
        let (nodes, arcs) = parse_xml("res/map2.xml");

        Self {
            nodes,
            arcs,
            mouse_pos: Pos2::new(0.0, 0.0),
            draw_ctx: DrawingContext::new(),
        }
    }
}

impl eframe::App for LuxembourgMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("Luxembourg Map");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut self.draw_ctx.zoom, 0.0..=1000.0)
                            .step_by(0.1)
                            .text("zoom"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.draw_ctx.pan_x, -1000.0..=1000.0)
                            .step_by(0.1)
                            .text("pan x"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.draw_ctx.pan_y, -1000.0..=1000.0)
                            .step_by(0.1)
                            .text("pan y"),
                    );
                    if ui
                        .button("Reset")
                        .on_hover_text("Reset zoom and pan")
                        .clicked()
                    {
                        self.draw_ctx.zoom = 100.0;
                        self.draw_ctx.pan_x = -700.0;
                        self.draw_ctx.pan_y = -179.0;
                    }
                });
            });

            ui.label(format!(
                "mouse pos: x: {}, y: {}",
                self.mouse_pos.x, self.mouse_pos.y
            ));

            ui.ctx().input(|i| {
                if i.pointer.is_decidedly_dragging() {
                    let delta = i.pointer.delta();

                    self.draw_ctx.pan_x += delta.x as f64 / self.draw_ctx.zoom;
                    self.draw_ctx.pan_y += delta.y as f64 / self.draw_ctx.zoom;
                }

                self.mouse_pos = i.pointer.interact_pos().unwrap_or_default();

                if i.scroll_delta.y != 0.0 {
                    let zoom_before = self.draw_ctx.zoom;
                    self.draw_ctx.zoom += i.scroll_delta.y as f64 / 10.0;

                    // Calculate the change in pan and divide it by 10
                    let new_pan_x = self.draw_ctx.pan_x * (self.draw_ctx.zoom / zoom_before);
                    let new_pan_y = self.draw_ctx.pan_y * (self.draw_ctx.zoom / zoom_before);

                    let screen_center = i.screen_rect().center();

                    let delta = (screen_center - self.mouse_pos) / 10.;

                    // Calculate the change in pan and divide it by 10
                    self.draw_ctx.pan_x +=
                        (new_pan_x - self.draw_ctx.pan_x + delta.x as f64) / self.draw_ctx.zoom;
                    self.draw_ctx.pan_y +=
                        (new_pan_y - self.draw_ctx.pan_y + delta.y as f64) / self.draw_ctx.zoom;

                    self.draw_ctx.pan_x = self.draw_ctx.pan_x.clamp(-1000.0, 1000.0);
                    self.draw_ctx.pan_y = self.draw_ctx.pan_y.clamp(-1000.0, 1000.0);
                }
            });
            // Draw arcs
            for arc in &self.arcs {
                self.draw_ctx.draw_arc(arc, ui, Color32::DARK_GRAY);
            }

            // Draw nodes
            let mut clicked_node_index = None;
            let mut node_drawings = Vec::new();

            for (i, node) in self.nodes.iter().enumerate() {
                if let Some(clicked_node_id) = self.draw_ctx.clicked_node_id.as_ref() {
                    if &node.id == clicked_node_id {
                        clicked_node_index = Some(i);
                        continue;
                    }
                }
                node_drawings.push((node, 0.5, ui.visuals().text_color()));
            }

            if let Some(index) = clicked_node_index {
                let node = &self.nodes[index];
                node_drawings.push((node, 3.0, Color32::RED));
            }

            for (node, size, color) in node_drawings {
                self.draw_ctx.draw_node(ui, node, size, color);
            }
        });
    }
}

struct DrawingContext {
    translator: Translator,
    clicked_node_id: Option<String>,
    zoom: f64,
    pan_x: f64,
    pan_y: f64,
}

impl DrawingContext {
    fn new() -> Self {
        Self {
            translator: Translator::default(),
            clicked_node_id: None,
            zoom: 100.0,
            pan_x: -700.0,
            pan_y: -179.0,
        }
    }

    fn draw_node(&mut self, ui: &mut Ui, node: &Node, r: f32, color: Color32) {
        let (x, y) = self.get_translation(node.longitude, node.latitude);

        // Create an interactable area for the circle with a unique ID
        let node_response = ui.interact(
            Rect::from_center_size(Pos2::new(x, y), Vec2::splat(r * 2.0)),
            Id::new(node.id.clone()),
            Sense::click(),
        );

        ui.painter().circle_filled(Pos2::new(x, y), r, color);

        if node_response.clicked() {
            println!("Node {} was clicked", node.id);
            self.clicked_node_id = Some(node.id.clone());
        }
    }

    fn draw_arc(&mut self, arc: &Arc, ui: &mut Ui, color: Color32) {
        let (from_x, from_y) = self.get_translation(arc.from_long, arc.from_lat);
        let (to_x, to_y) = self.get_translation(arc.to_long, arc.to_lat);

        ui.painter().line_segment(
            [Pos2::new(from_x, from_y), Pos2::new(to_x, to_y)],
            (0.5, color),
        );
    }

    fn get_translation(&mut self, longitude: f64, latitude: f64) -> TranslationResults {
        self.translator
            .project(longitude, latitude, self.zoom, self.pan_x, self.pan_y)
    }
}
