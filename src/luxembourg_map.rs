use egui::{Color32, Pos2};
use log::debug;
use std::sync::mpsc::Sender;

use crate::{
    contexts::{DataContext, DrawingContext},
    models::{Edge, Node},
    parser::parse_xml,
    translator::Translator,
    utils::constants::{DEFAULT_PAN, DEFAULT_ZOOM, MAX_PAN, MIN_PAN},
};

pub struct LuxembourgMap {
    data_ctx: DataContext,
    mouse_pos: Pos2,
    draw_ctx: DrawingContext,
    data_loaded: bool,
}

impl LuxembourgMap {
    pub fn new() -> Self {
        Self {
            data_ctx: DataContext::default(),
            mouse_pos: Pos2::new(0.0, 0.0),
            draw_ctx: DrawingContext::new(),
            data_loaded: false,
        }
    }
}

impl eframe::App for LuxembourgMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.data_ctx.rx_nodes.try_recv() {
            Ok(nodes) => {
                self.data_ctx.nodes = nodes;
                self.data_loaded = true;
                debug!("Nodes received");
            }
            _ => {
                self.data_loaded = false;
            }
        }
        match self.data_ctx.rx_edges.try_recv() {
            Ok(edges) => {
                self.data_ctx.edges = edges;
                self.data_loaded = true;
                debug!("Edges received");
            }
            _ => {
                self.data_loaded = false;
            }
        }

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
                        egui::Slider::new(&mut self.draw_ctx.pan.x, -1000.0..=1000.0)
                            .step_by(0.1)
                            .text("pan x"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.draw_ctx.pan.y, -1000.0..=1000.0)
                            .step_by(0.1)
                            .text("pan y"),
                    );
                    if ui
                        .button("Load")
                        .on_hover_text("Load the Luxembourg map")
                        .clicked()
                    {
                        send_parse_request(
                            self.data_ctx.tx_nodes.clone(),
                            self.data_ctx.tx_edges.clone(),
                            self.draw_ctx.translator.clone(),
                            ctx.clone(),
                        );
                        debug!("Loading map...")
                    }
                    if ui
                        .button("Reset")
                        .on_hover_text("Reset zoom and pan")
                        .clicked()
                    {
                        self.draw_ctx.zoom = DEFAULT_ZOOM;
                        self.draw_ctx.pan = DEFAULT_PAN;
                    }
                    ui.label(self.data_loaded.to_string());
                });
            });

            ui.label(format!(
                "mouse pos: x: {}, y: {}",
                self.mouse_pos.x, self.mouse_pos.y
            ));

            ui.ctx().input(|i| {
                if i.pointer.is_decidedly_dragging() {
                    let delta = i.pointer.delta();

                    self.draw_ctx.pan.x += delta.x / self.draw_ctx.zoom;
                    self.draw_ctx.pan.y += delta.y / self.draw_ctx.zoom;
                }

                self.mouse_pos = i.pointer.interact_pos().unwrap_or_default();

                if i.scroll_delta.y != 0.0 {
                    let zoom_before = self.draw_ctx.zoom;
                    self.draw_ctx.zoom += i.scroll_delta.y / 10.0;

                    if self.draw_ctx.zoom < 1.0 {
                        self.draw_ctx.zoom = 1.0;
                    }

                    let new_pan = self.draw_ctx.pan * (self.draw_ctx.zoom / zoom_before);

                    let screen_center = i.screen_rect().center();

                    let delta = (screen_center - self.mouse_pos) / 10.;

                    self.draw_ctx.pan += (new_pan - self.draw_ctx.pan + delta) / self.draw_ctx.zoom;
                    self.draw_ctx.pan = self.draw_ctx.pan.clamp(MIN_PAN, MAX_PAN);
                }
            });

            // Draw edges
            for edge in &self.data_ctx.edges {
                self.draw_ctx.draw_edge(edge, ui, Color32::DARK_GRAY);
            }

            // Draw nodes
            let mut clicked_node_idx = None;
            let mut node_drawings = Vec::new();

            for (idx, node) in self.data_ctx.nodes.iter().enumerate() {
                if let Some(clicked_node_id) = self.draw_ctx.clicked_node_id.as_ref() {
                    if &node.id == clicked_node_id {
                        clicked_node_idx = Some(idx);
                        continue;
                    }
                }
                node_drawings.push((node, 0.5, ui.visuals().text_color()));
            }

            if let Some(idx) = clicked_node_idx {
                let node = &self.data_ctx.nodes[idx];
                node_drawings.push((node, 3.0, Color32::RED));
            }

            for (node, size, color) in node_drawings {
                self.draw_ctx.draw_node(ui, node, size, color);
            }
        });
    }
}

fn send_parse_request(
    tx_nodes: Sender<Vec<Node>>,
    tx_edges: Sender<Vec<Edge>>,
    translator: Translator,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        debug!("Parsing map...");
        let (nodes, edges) = parse_xml("res/map2.xml", translator);
        debug!("Map parsed");

        debug!("Sending nodes...");
        tx_nodes.send(nodes).unwrap();
        debug!("Nodes sent");

        debug!("Sending edges...");
        tx_edges.send(edges).unwrap();
        debug!("Edges sent");

        debug!("Map loaded");
        ctx.request_repaint();
    });
}
