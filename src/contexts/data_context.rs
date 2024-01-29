use std::sync::mpsc::{Receiver, Sender};

use crate::{
    models::{Edge, Node},
    utils::constants::xml_files::{MAP2_XML, TEST_XML},
};

pub struct DataContext {
    pub rx_nodes: Receiver<Vec<Node>>,
    pub rx_edges: Receiver<Vec<Edge>>,

    pub tx_nodes: Sender<Vec<Node>>,
    pub tx_edges: Sender<Vec<Edge>>,

    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,

    pub nodes_loading: bool,
    pub edges_loading: bool,

    data_file: String,

    first_load: std::cell::Cell<bool>,
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
            nodes_loading: false,
            edges_loading: false,
            data_file: MAP2_XML.to_string(),
            first_load: std::cell::Cell::new(true),
        }
    }

    pub fn data_file(&self) -> &str {
        &self.data_file
    }

    pub fn switch_data_file(&mut self) {
        self.data_file = if self.data_file == MAP2_XML {
            TEST_XML.to_string()
        } else {
            MAP2_XML.to_string()
        };
    }

    pub fn has_data(&self) -> bool {
        !self.nodes.is_empty() && !self.edges.is_empty()
    }

    pub fn first_load(&self) -> bool {
        // short circuit after first load
        if self.first_load.get() {
            self.first_load.set(false);
            true
        } else {
            false
        }
    }
}

impl Default for DataContext {
    fn default() -> Self {
        Self::empty()
    }
}
