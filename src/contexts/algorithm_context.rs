use egui::Pos2;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{
    models::{Edge, Node},
    utils::FloatOrd,
};

pub struct AlgorithmContext {
    selected_edges: HashSet<Edge>,
    current_computation_args: Option<(Node, Node)>,
    computed_runs: HashMap<(Node, Node), HashSet<Edge>>,
}

impl AlgorithmContext {
    pub fn new() -> Self {
        Self {
            selected_edges: HashSet::new(),
            current_computation_args: None,
            computed_runs: HashMap::new(),
        }
    }

    pub fn is_edge_selected(&self, edge: &Edge) -> bool {
        self.selected_edges.contains(edge)
    }

    pub fn is_new_args(&self, start: &Node, end: &Node) -> bool {
        match self.current_computation_args.as_ref() {
            None => true,
            Some((s, e)) => s != start || e != end,
        }
    }

    pub fn compute_path(&mut self, from: &Node, to: &Node, neighboors: &HashMap<Node, Vec<Edge>>) {
        if !self.is_new_args(from, to) {
            return;
        }
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.computed_runs.entry((from.clone(), to.clone()))
        {
            let selected_edges = run_astar(from, to, neighboors);
            e.insert(selected_edges);
        }
        self.selected_edges = self
            .computed_runs
            .get(&(from.clone(), to.clone()))
            .unwrap()
            .clone();
        self.current_computation_args = Some((from.clone(), to.clone()));
    }
}

fn run_astar(start: &Node, end: &Node, neighboors: &HashMap<Node, Vec<Edge>>) -> HashSet<Edge> {
    let mut frontier: PriorityQueue<NodeData, Reverse<FloatOrd<f32>>> = PriorityQueue::new();
    frontier.push(NodeData::from(start.clone()), Reverse(FloatOrd(0.0)));

    let mut came_from: HashMap<NodeData, Option<NodeData>> = HashMap::new();
    let mut cost_so_far: HashMap<NodeData, FloatOrd<f32>> = HashMap::new();

    came_from.insert(NodeData::from(start.clone()), None);
    cost_so_far.insert(NodeData::from(start.clone()), FloatOrd(0.0));

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap().0;

        if current.node == *end {
            break;
        }

        for next in neighboors.get(&current.node).unwrap() {
            let new_cost = *cost_so_far.get(&current).unwrap() + FloatOrd(next.length);

            let next_node_data = NodeData::from(next.to.clone());
            if !cost_so_far.contains_key(&next_node_data)
                || new_cost < *cost_so_far.get(&next_node_data).unwrap()
            {
                cost_so_far.insert(next_node_data.clone(), new_cost);
                let priority = new_cost + heuristic(&next.to, end);
                frontier.push(next_node_data.clone(), Reverse(priority));
                came_from.insert(next_node_data.clone(), Some(current.clone()));
            }
        }
    }

    reconstruct_path(&came_from, start, end)
}

fn reconstruct_path(
    came_from: &HashMap<NodeData, Option<NodeData>>,
    start: &Node,
    end: &Node,
) -> HashSet<Edge> {
    let _str = format!("{:?}", came_from);
    came_from.iter().for_each(|(k, v)| {
        println!("{:?} <-> {:?}", k, v);
    });

    let mut selected_edges = HashSet::new();
    let mut current = NodeData::from(end.clone());

    while current.node != *start {
        let next = came_from.get(&current).unwrap().clone().unwrap();
        selected_edges.insert(Edge::new(next.node.clone(), current.node.clone(), 0.0));
        current = next.clone();
    }

    selected_edges
}

fn heuristic(a: &Node, b: &Node) -> FloatOrd<f32> {
    distance(a.position, b.position)
}

fn distance(a: Pos2, b: Pos2) -> FloatOrd<f32> {
    FloatOrd(((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt())
}

#[derive(Debug, Clone)]
struct NodeData {
    node: Node,
    cost: FloatOrd<f32>,
}

impl NodeData {
    fn new(node: Node, cost: FloatOrd<f32>) -> Self {
        Self { node, cost }
    }

    fn from(node: Node) -> Self {
        Self::new(node, FloatOrd(0.0))
    }

    fn from_with_cost(node: Node, cost: FloatOrd<f32>) -> Self {
        Self::new(node, cost)
    }

    fn from_with_parent(node: Node) -> Self {
        Self::new(node, FloatOrd(0.0))
    }

    fn from_with_cost_and_parent(node: Node, cost: FloatOrd<f32>) -> Self {
        Self::new(node, cost)
    }
}

impl Hash for NodeData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for NodeData {}

impl PartialOrd for NodeData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
