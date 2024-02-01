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

type RunArgs = (Node, Node, bool, bool); // (start, end, is_marking_passed_edges, use_heuristic)
type RunOutput = (HashSet<Edge>, HashSet<Edge>); // (selected_edges, passed_edges)

pub struct AlgorithmContext {
    pub is_marking_passed_edges: bool,
    pub use_astar: bool,
    selected_edges: HashSet<Edge>,
    passed_edges: HashSet<Edge>,
    current_run_args: Option<RunArgs>,
    computed_runs: HashMap<RunArgs, RunOutput>, // (start, end, is_marking_passed_edges) -> (selected_edges, passed_edges)
}

impl AlgorithmContext {
    pub fn new() -> Self {
        Self {
            is_marking_passed_edges: false,
            use_astar: true,
            selected_edges: HashSet::new(),
            passed_edges: HashSet::new(),
            current_run_args: None,
            computed_runs: HashMap::new(),
        }
    }

    pub fn is_edge_selected(&self, edge: &Edge) -> bool {
        self.selected_edges.contains(edge)
    }

    pub fn is_edge_passed(&self, edge: &Edge) -> bool {
        self.passed_edges.contains(edge)
    }

    pub fn is_new_args(&self, start: &Node, end: &Node) -> bool {
        match self.current_run_args.as_ref() {
            None => true,
            Some((s, e, is_marking, uses_astar)) => {
                s != start
                    || e != end
                    || *is_marking != self.is_marking_passed_edges
                    || *uses_astar != self.use_astar
            }
        }
    }

    pub fn compute_path(&mut self, from: &Node, to: &Node, neighbors: &HashMap<Node, Vec<Edge>>) {
        if !self.is_new_args(from, to) {
            return;
        }
        if let std::collections::hash_map::Entry::Vacant(e) = self.computed_runs.entry((
            from.clone(),
            to.clone(),
            self.is_marking_passed_edges,
            self.use_astar,
        )) {
            let (selected_edges, passed_edges) = run_pathfinding_algorithm(
                from,
                to,
                neighbors,
                self.is_marking_passed_edges,
                self.use_astar,
            );
            e.insert((selected_edges, passed_edges));
        }
        (self.selected_edges, self.passed_edges) = self
            .computed_runs
            .get(&(
                from.clone(),
                to.clone(),
                self.is_marking_passed_edges,
                self.use_astar,
            ))
            .unwrap()
            .clone();
        self.current_run_args = Some((
            from.clone(),
            to.clone(),
            self.is_marking_passed_edges,
            self.use_astar,
        ));
    }
}

fn run_pathfinding_algorithm(
    start: &Node,
    end: &Node,
    neighbors: &HashMap<Node, Vec<Edge>>,
    mark_passed_edges: bool,
    use_heuristic: bool,
) -> RunOutput {
    let mut passed_edges = HashSet::new();

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

        for next in neighbors.get(&current.node).unwrap() {
            if mark_passed_edges {
                passed_edges.insert(next.clone());
            }

            let new_cost = *cost_so_far.get(&current).unwrap() + FloatOrd(next.length);

            let next_node_data = NodeData::from(next.to.clone());
            if !cost_so_far.contains_key(&next_node_data)
                || new_cost < *cost_so_far.get(&next_node_data).unwrap()
            {
                cost_so_far.insert(next_node_data.clone(), new_cost);

                let priority = if use_heuristic {
                    heuristic(&next.to, end)
                } else {
                    new_cost
                };

                frontier.push(next_node_data.clone(), Reverse(priority));
                came_from.insert(next_node_data.clone(), Some(current.clone()));
            }
        }
    }

    (reconstruct_path(&came_from, start, end), passed_edges)
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

const HEURISTIC_MULTIPLIER: FloatOrd<f32> = FloatOrd(1.0);
fn heuristic(a: &Node, b: &Node) -> FloatOrd<f32> {
    HEURISTIC_MULTIPLIER * distance(&a.position, &b.position)
}

fn distance(a: &Pos2, b: &Pos2) -> FloatOrd<f32> {
    FloatOrd(((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt())
}

// fn manhattan_distance(a: &Pos2, b: &Pos2) -> FloatOrd<f32> {
//     let dx = (a.x - b.x).abs();
//     let dy = (a.y - b.y).abs();
//     FloatOrd(dx + dy)
// }

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
