use egui::{Color32, Id, Pos2, Rect, Sense, Ui, Vec2};
use log::debug;
use std::sync::mpsc::{Receiver, Sender};

use crate::{edge::Edge, node::Node, parser::parse_xml, translator::Translator};

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
                        self.draw_ctx.zoom = 100.0;
                        self.draw_ctx.pan_x = -700.0;
                        self.draw_ctx.pan_y = -179.0;
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

                    self.draw_ctx.pan_x += delta.x / self.draw_ctx.zoom;
                    self.draw_ctx.pan_y += delta.y / self.draw_ctx.zoom;
                }

                self.mouse_pos = i.pointer.interact_pos().unwrap_or_default();

                if i.scroll_delta.y != 0.0 {
                    let zoom_before = self.draw_ctx.zoom;
                    self.draw_ctx.zoom += i.scroll_delta.y / 10.0;

                    if self.draw_ctx.zoom < 1.0 {
                        self.draw_ctx.zoom = 1.0;
                    }

                    // Calculate the change in pan and divide it by 10
                    let new_pan_x = self.draw_ctx.pan_x * (self.draw_ctx.zoom / zoom_before);
                    let new_pan_y = self.draw_ctx.pan_y * (self.draw_ctx.zoom / zoom_before);

                    let screen_center = i.screen_rect().center();

                    let delta = (screen_center - self.mouse_pos) / 10.;

                    // Calculate the change in pan and divide it by 10
                    self.draw_ctx.pan_x +=
                        (new_pan_x - self.draw_ctx.pan_x + delta.x) / self.draw_ctx.zoom;
                    self.draw_ctx.pan_y +=
                        (new_pan_y - self.draw_ctx.pan_y + delta.y) / self.draw_ctx.zoom;

                    self.draw_ctx.pan_x = self.draw_ctx.pan_x.clamp(-1000.0, 1000.0);
                    self.draw_ctx.pan_y = self.draw_ctx.pan_y.clamp(-1000.0, 1000.0);
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
            // debug!("Map drawn");
            // } else {
            //     ui.add(egui::ProgressBar::new(100.0).animate(true));
            // }
        });
    }
}

struct DataContext {
    rx_nodes: Receiver<Vec<Node>>,
    rx_edges: Receiver<Vec<Edge>>,

    tx_nodes: Sender<Vec<Node>>,
    tx_edges: Sender<Vec<Edge>>,

    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl DataContext {
    fn empty() -> Self {
        Self::new(vec![], vec![])
    }

    fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        let (tx_nodes, rx_nodes) = std::sync::mpsc::channel();
        let (tx_edges, rx_edges) = std::sync::mpsc::channel();

        Self {
            rx_nodes,
            rx_edges,
            tx_nodes,
            tx_edges,
            nodes,
            edges,
        }
    }
}

impl Default for DataContext {
    fn default() -> Self {
        Self::empty()
    }
}

struct DrawingContext {
    translator: Translator,
    clicked_node_id: Option<String>,
    zoom: f32,
    pan_x: f32,
    pan_y: f32,
}

impl DrawingContext {
    fn new() -> Self {
        Self {
            translator: Translator::default(),
            clicked_node_id: None,
            zoom: 100.,
            pan_x: -700.,
            pan_y: -179.,
        }
    }

    fn draw_node(&mut self, ui: &mut Ui, node: &Node, r: f32, color: Color32) {
        let position_on_screen = (node.position + Vec2::new(self.pan_x, self.pan_y)) * self.zoom;

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

    fn draw_edge(&mut self, edge: &Edge, ui: &mut Ui, color: Color32) {
        let from_position = (edge.from + Vec2::new(self.pan_x, self.pan_y)) * self.zoom;
        let to_position = (edge.to + Vec2::new(self.pan_x, self.pan_y)) * self.zoom;

        debug!("Drawing edge from {:?} to {:?}", from_position, to_position);

        ui.painter()
            .line_segment([from_position, to_position], (0.5, color));
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
