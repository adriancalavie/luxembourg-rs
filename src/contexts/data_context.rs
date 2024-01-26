use std::sync::mpsc::{Receiver, Sender};

use crate::models::{Edge, Node};

pub struct DataContext {
    pub rx_nodes: Receiver<Vec<Node>>,
    pub rx_edges: Receiver<Vec<Edge>>,

    pub tx_nodes: Sender<Vec<Node>>,
    pub tx_edges: Sender<Vec<Edge>>,

    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
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

    pub fn is_loaded(&self) -> bool {
        !self.nodes.is_empty() && !self.edges.is_empty()
    }
}

impl Default for DataContext {
    fn default() -> Self {
        Self::empty()
    }
}
