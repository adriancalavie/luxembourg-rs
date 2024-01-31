use egui::{Align2, Color32, Id, Pos2, Rect, Sense, Vec2};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use log::debug;
use std::{collections::HashMap, sync::mpsc::Sender};

use crate::{
    components::toggle_ui,
    contexts::{DataContext, DrawingContext},
    models::{Edge, Node},
    parser::parse_xml,
    utils::{
        constants::{DEFAULT_PAN, DEFAULT_ZOOM, MAX_PAN, MAX_ZOOM, MIN_PAN},
        FrameHistory,
    },
};

pub struct Map {
    data_ctx: DataContext,
    draw_ctx: DrawingContext,
    state: UIState,
}

impl Map {
    pub fn new() -> Self {
        Self {
            data_ctx: DataContext::default(),
            draw_ctx: DrawingContext::new(),
            state: UIState::default(),
        }
    }

    fn is_selected(&self, node_id: &str) -> bool {
        if let Some(start_node) = &self.state.start_node {
            if start_node.id == node_id {
                return true;
            }
        }
        if let Some(end_node) = &self.state.end_node {
            if end_node.id == node_id {
                return true;
            }
        }
        false
    }

    fn send_load_data_req(&mut self, ctx: &egui::Context) {
        self.data_ctx.nodes_loading = true;
        self.data_ctx.edges_loading = true;

        send_parse_request(
            self.data_ctx.tx_nodes.clone(),
            self.data_ctx.tx_edges.clone(),
            self.data_ctx.tx_neighboors.clone(),
            self.data_ctx.data_file().to_string(),
            ctx.clone(),
        );
    }

    fn render_edges(&self, ui: &mut egui::Ui) {
        if !self.data_ctx.has_data() {
            return;
        }

        let edge_color = if ui.visuals().dark_mode {
            Color32::DARK_GRAY
        } else {
            Color32::GRAY
        };

        self.data_ctx.edges.iter().for_each(|edge| {
            let (from, to) = self.draw_ctx.calc_edge_coords(edge);
            {
                ui.painter().line_segment([from, to], (0.5, edge_color));
            }
        });
    }

    fn render_nodes(&mut self, ui: &mut egui::Ui) {
        let mut selected_nodes = Vec::new();
        let mut nodes_to_draw = Vec::new();
        let mut nodes_to_select = Vec::new();

        self.data_ctx.nodes.iter().for_each(|node| {
            if self.is_selected(&node.id) {
                selected_nodes.push((node, 3.0, Color32::RED));
                return;
            }
            nodes_to_draw.push((node, 0.5, ui.visuals().text_color()));
        });

        nodes_to_draw.extend(selected_nodes);
        assert_eq!(
            nodes_to_draw.len(),
            self.data_ctx.nodes.len(),
            "nodes_to_draw count does not match data_ctx.nodes count"
        );

        for (node, size, color) in nodes_to_draw.into_iter() {
            let position_on_screen = self.draw_ctx.calc_node_coords(node);
            // Create an interactable area for the circle with a unique ID
            let node_hook = ui.interact(
                Rect::from_center_size(position_on_screen, Vec2::splat(size * 2.0)),
                Id::new(node.id.clone()),
                Sense::click(),
            );

            if node_hook.clicked() {
                nodes_to_select.push(node.clone());
            }
            ui.painter().circle_filled(position_on_screen, size, color);
        }

        nodes_to_select.into_iter().for_each(|node| {
            self.select_node(node);
        });
    }

    fn select_node(&mut self, node: Node) {
        println!("Node {} was clicked", node);
        let node_id = node.id.clone();
        match (&self.state.start_node, &self.state.end_node) {
            (None, _) => {
                // start node is not set
                self.state.start_node = Some(node);
                self.state.show_toast(
                    format!("Node {} selected as 'start'", &node_id),
                    ToastKind::Info,
                );
            }
            (Some(start_node), None) => {
                // end node is not set
                if start_node == &node {
                    // user clicked on the start node
                    self.state.start_node = None;
                    self.state
                        .show_toast(format!("Node {} unselected", &node_id), ToastKind::Info);
                } else {
                    // user clicked on a new node
                    self.state.end_node = Some(node);
                    self.state.show_toast(
                        format!("Node {} selected as 'end'", &node_id),
                        ToastKind::Info,
                    );
                }
            }
            (Some(start_id), Some(end_id)) => {
                if start_id == &node {
                    // user clicked on the start node
                    self.state.start_node = None;
                    self.state
                        .show_toast(format!("Node {} unselected", &node_id), ToastKind::Info);
                } else if end_id == &node {
                    // user clicked on the end node
                    self.state.end_node = None;
                    self.state
                        .show_toast(format!("Node {} unselected", &node_id), ToastKind::Info);
                } else {
                    // user clicked on a new node
                    self.state.start_node = Some(node);
                    self.state.end_node = None;
                    self.state.show_toast(
                        format!("Node {} selected as 'start'", &node_id),
                        ToastKind::Info,
                    );
                }
            }
        }
    }

    fn render_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Luxembourg Map");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.draw_ctx.zoom, 0.0..=MAX_ZOOM)
                        .step_by(0.1)
                        .text("Zoom"),
                );
                ui.add(
                    egui::Slider::new(&mut self.draw_ctx.pan.x, MIN_PAN.x..=MAX_PAN.x)
                        .step_by(0.1)
                        .text("Pan x"),
                );
                ui.add(
                    egui::Slider::new(&mut self.draw_ctx.pan.y, MIN_PAN.y..=MAX_PAN.y)
                        .step_by(0.1)
                        .text("Pan y"),
                );
                ui.label(format!("Selected data file: {}", self.data_ctx.data_file()));
                let toggle_resp = toggle_ui(ui, &mut self.state.test_data_on);
                if toggle_resp.clicked() {
                    self.data_ctx.switch_data_file();
                    self.send_load_data_req(ctx);
                }
                if ui
                    .button("Reset zoom and pan")
                    .on_hover_text("Reset zoom and pan")
                    .clicked()
                {
                    self.draw_ctx.zoom = DEFAULT_ZOOM;
                    self.draw_ctx.pan = DEFAULT_PAN;
                }
                ui.label(format!("Data loaded: {}", self.data_ctx.has_data()));
                ui.label(format!("FPS: {:.1}", self.state.frame_history.fps()));
                self.state.frame_history.ui(ui);

                if self.data_ctx.nodes_loading {
                    ui.label("Loading nodes...");
                }
                if self.data_ctx.edges_loading {
                    ui.label("Loading edges...");
                }
            });
        });

        ui.label(format!(
            "mouse pos: x: {}, y: {}",
            self.state.mouse_pos.x, self.state.mouse_pos.y
        ));

        ui.ctx().input(|i| {
            if i.pointer.is_decidedly_dragging() {
                let delta = i.pointer.delta();

                self.draw_ctx.pan.x += delta.x / self.draw_ctx.zoom;
                self.draw_ctx.pan.y += delta.y / self.draw_ctx.zoom;
            }

            self.state.mouse_pos = i.pointer.interact_pos().unwrap_or_default();

            if i.scroll_delta.y != 0.0 {
                let zoom_before = self.draw_ctx.zoom;
                self.draw_ctx.zoom += i.scroll_delta.y / 10.0;

                if self.draw_ctx.zoom < 1.0 {
                    self.draw_ctx.zoom = 1.0;
                }

                let new_pan = self.draw_ctx.pan * (self.draw_ctx.zoom / zoom_before);

                let screen_center = i.screen_rect().center();

                let delta = (screen_center - self.state.mouse_pos) / 10.;

                self.draw_ctx.pan += (new_pan - self.draw_ctx.pan + delta) / self.draw_ctx.zoom;
                self.draw_ctx.pan = self.draw_ctx.pan.clamp(MIN_PAN, MAX_PAN);
            }
        });
    }

    fn try_initialize_data(&mut self, ctx: &egui::Context) {
        if self.data_ctx.first_load() {
            self.send_load_data_req(ctx);
        }
    }

    fn check_for_data_updates(&mut self) {
        if let Ok(nodes) = self.data_ctx.rx_nodes.try_recv() {
            self.data_ctx.nodes = nodes;
            debug!("Nodes received");
            self.state.show_toast(
                format!("Nodes received: {}", self.data_ctx.nodes.len()),
                ToastKind::Info,
            );
            self.data_ctx.nodes_loading = false;
        }
        if let Ok(edges) = self.data_ctx.rx_edges.try_recv() {
            self.data_ctx.edges = edges;
            debug!("Edges received");
            self.state.show_toast(
                format!("Edges received: {}", self.data_ctx.edges.len()),
                ToastKind::Info,
            );
            self.data_ctx.edges_loading = false;
        }
        if let Ok(neighboors) = self.data_ctx.rx_neighboors.try_recv() {
            self.data_ctx.neighboors = neighboors;
            debug!("Neighboors received");
            self.state.show_toast(
                format!("Neighboors received: {}", self.data_ctx.neighboors.len()),
                ToastKind::Info,
            );
        }
        if self.state.is_start_and_end_set() {
            debug!("Start and end nodes are set");
            self.state.show_toast(
                format!(
                    "Start and end nodes are set: {} and {}",
                    self.state.start_node.as_ref().unwrap(),
                    self.state.end_node.as_ref().unwrap()
                ),
                ToastKind::Info,
            );
        }
    }

    fn update_fps(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.state
            .frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
    }

    fn render_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // Draw controls
            self.render_controls(ui, ctx);
            // Draw edges
            self.render_edges(ui);
            // Draw nodes
            self.render_nodes(ui);

            // Draw toasts
            self.state.toasts.show(ctx);
        });
    }
}

impl eframe::App for Map {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_fps(ctx, frame);

        self.try_initialize_data(ctx);
        self.check_for_data_updates();

        self.render_ui(ctx);
    }
}

struct UIState {
    test_data_on: bool,
    start_node: Option<Node>,
    end_node: Option<Node>,
    frame_history: FrameHistory,
    mouse_pos: Pos2,
    toasts: Toasts,
}

impl UIState {
    fn show_toast(&mut self, message: String, toast_type: ToastKind) {
        self.toasts.add(Toast {
            text: message.into(),
            kind: toast_type,
            options: ToastOptions::default()
                .duration_in_seconds(5.0)
                .show_progress(true)
                .show_icon(true),
        });
    }

    fn is_start_and_end_set(&self) -> bool {
        self.start_node.is_some() && self.end_node.is_some()
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            test_data_on: false,
            start_node: None,
            end_node: None,
            frame_history: FrameHistory::default(),
            mouse_pos: Pos2::new(0.0, 0.0),
            toasts: Toasts::new()
                .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
                .direction(egui::Direction::TopDown),
        }
    }
}

fn send_parse_request(
    tx_nodes: Sender<Vec<Node>>,
    tx_edges: Sender<Vec<Edge>>,
    tx_neighboors: Sender<HashMap<Node, Vec<Node>>>,
    file: String,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        debug!("Parsing map...");
        let (nodes, edges, neighboors) = parse_xml(&file);
        debug!("Map parsed");

        debug!("Sending nodes...");
        tx_nodes.send(nodes).unwrap();
        debug!("Nodes sent");

        debug!("Sending edges...");
        tx_edges.send(edges).unwrap();
        debug!("Edges sent");

        debug!("Sending neighboors...");
        tx_neighboors.send(neighboors).unwrap();
        debug!("Neighboors sent");

        debug!("Map loaded");
        ctx.request_repaint();
    });
}
