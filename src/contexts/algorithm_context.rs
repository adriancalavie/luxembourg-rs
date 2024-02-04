use egui::Pos2;
use enum_iterator::Sequence;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

use crate::{
    models::{Edge, Node},
    utils::FloatOrd,
};

// this is an arbitrary value found by trial and error
const MULTIPLICITY_BASE: FloatOrd<f32> = FloatOrd(13_000.);

// (start, end, is_marking_passed_edges, algorithm_type, astar_weight, use_manhattan)
type RunArgs = (Node, Node, bool, AlgorithmType, FloatOrd<f32>, bool);
// note: the assumption is that the neighbors will not change during the lifetime of the context
//      if the edges would change between runs, the neighbors hashmap should also be included in the RunArgs

// (selected_edges, passed_edges, total_cost)
type RunOutput = (HashSet<Edge>, HashSet<Edge>, f32);

#[derive(PartialEq, Eq, Hash, Sequence, Copy, Clone, Debug)]
pub enum AlgorithmType {
    AStar,
    HybridAStar,
    Dijkstra,
}

impl fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmType::AStar => write!(f, "A Star"),
            AlgorithmType::HybridAStar => write!(f, "Hybrid A Star"),
            AlgorithmType::Dijkstra => write!(f, "Dijkstra"),
        }
    }
}

pub struct AlgorithmContext {
    pub is_marking_passed_edges: bool,
    pub algorithm_type: AlgorithmType,
    pub astar_weight: FloatOrd<f32>,
    pub total_cost: f32,
    pub use_manhattan: bool,
    selected_edges: HashSet<Edge>,
    passed_edges: HashSet<Edge>,
    current_run_args: Option<RunArgs>,
    computed_runs: HashMap<RunArgs, RunOutput>, // (start, end, is_marking_passed_edges) -> (selected_edges, passed_edges)
}

impl AlgorithmContext {
    pub fn new() -> Self {
        Self {
            is_marking_passed_edges: false,
            algorithm_type: AlgorithmType::HybridAStar,
            astar_weight: FloatOrd(1.0),
            total_cost: 0.0,
            use_manhattan: true,
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
            Some((s, e, is_marking, algorithm_type, astar_weight, use_manhattan)) => {
                s != start
                    || e != end
                    || *is_marking != self.is_marking_passed_edges
                    || *algorithm_type != self.algorithm_type
                    || *astar_weight != self.astar_weight
                    || *use_manhattan != self.use_manhattan
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
            self.algorithm_type,
            self.astar_weight,
            self.use_manhattan,
        )) {
            let run_output = run_pathfinding_algorithm(
                from,
                to,
                neighbors,
                self.is_marking_passed_edges,
                self.algorithm_type,
                self.astar_weight,
                self.use_manhattan,
            );
            e.insert(run_output);
        }
        // write the run outputs from the computed_runs hashmap into the context
        (self.selected_edges, self.passed_edges, self.total_cost) = self
            .computed_runs
            .get(&(
                from.clone(),
                to.clone(),
                self.is_marking_passed_edges,
                self.algorithm_type,
                self.astar_weight,
                self.use_manhattan,
            ))
            .unwrap()
            .clone();
        // update the current_run_args
        self.current_run_args = Some((
            from.clone(),
            to.clone(),
            self.is_marking_passed_edges,
            self.algorithm_type,
            self.astar_weight,
            self.use_manhattan,
        ));
    }

    pub fn is_using_astar(&self) -> bool {
        self.algorithm_type == AlgorithmType::AStar
            || self.algorithm_type == AlgorithmType::HybridAStar
    }
}

fn run_pathfinding_algorithm(
    start: &Node,
    end: &Node,
    neighbors: &HashMap<Node, Vec<Edge>>,
    mark_passed_edges: bool,
    algorithm_type: AlgorithmType,
    heuristic_weight: FloatOrd<f32>,
    use_manhattan: bool,
) -> RunOutput {
    let mut passed_edges = HashSet::new();
    let mut total_cost: f32 = 0.0;

    let mut frontier: PriorityQueue<NodeData, Reverse<FloatOrd<f32>>> = PriorityQueue::new();
    frontier.push(NodeData::from(start.clone()), Reverse(FloatOrd(0.0)));

    let mut came_from: HashMap<NodeData, Option<NodeData>> = HashMap::new();
    let mut cost_so_far: HashMap<NodeData, FloatOrd<f32>> = HashMap::new();

    came_from.insert(NodeData::from(start.clone()), None);
    cost_so_far.insert(NodeData::from(start.clone()), FloatOrd(0.0));

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap().0;

        if current.node == *end {
            total_cost = cost_so_far.get(&current).map(|f| f.0).unwrap_or(0.0);
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

                let priority = match algorithm_type {
                    AlgorithmType::AStar => heuristic(&next.to, end, None, use_manhattan),
                    AlgorithmType::HybridAStar => {
                        new_cost + heuristic(&next.to, end, Some(heuristic_weight), use_manhattan)
                    }
                    AlgorithmType::Dijkstra => new_cost,
                };

                frontier.push(next_node_data.clone(), Reverse(priority));
                came_from.insert(next_node_data.clone(), Some(current.clone()));
            }
        }
    }

    (
        reconstruct_path(&came_from, start, end),
        passed_edges,
        total_cost,
    )
}

fn reconstruct_path(
    came_from: &HashMap<NodeData, Option<NodeData>>,
    start: &Node,
    end: &Node,
) -> HashSet<Edge> {
    let _str = format!("{:?}", came_from);

    let mut selected_edges = HashSet::new();
    let mut current = NodeData::from(end.clone());

    while current.node != *start {
        let next = came_from.get(&current).unwrap().clone().unwrap();
        selected_edges.insert(Edge::new(next.node.clone(), current.node.clone(), 0.0));
        current = next.clone();
    }

    selected_edges
}

fn heuristic(
    a: &Node,
    b: &Node,
    multiplier: Option<FloatOrd<f32>>,
    use_manhattan: bool,
) -> FloatOrd<f32> {
    // Apply a base multiplicity to make it more aggressive by default
    // Also, the user can set 'simple' values like 1.5, 2.0, etc instead of 19_500.0, 26_000.0, etc
    let mult = multiplier.unwrap_or(FloatOrd(1.0)) * MULTIPLICITY_BASE;
    mult * distance(&a.position, &b.position, use_manhattan)
}

fn distance(a: &Pos2, b: &Pos2, use_manhattan: bool) -> FloatOrd<f32> {
    if use_manhattan {
        manhattan_distance(a, b)
    } else {
        euclidean_distance(a, b)
    }
}

fn euclidean_distance(a: &Pos2, b: &Pos2) -> FloatOrd<f32> {
    FloatOrd(((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt())
}

fn manhattan_distance(a: &Pos2, b: &Pos2) -> FloatOrd<f32> {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    FloatOrd(dx + dy)
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
