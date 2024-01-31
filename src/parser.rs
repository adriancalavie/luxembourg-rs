use std::collections::HashMap;

use crate::{
    models::{Edge, Node},
    translator::TRANSLATOR,
};

pub fn parse_xml(file_name: &str) -> (Vec<Node>, Vec<Edge>, HashMap<Node, Vec<Edge>>) {
    let text = std::fs::read_to_string(file_name).unwrap();

    let doc = roxmltree::Document::parse(&text).unwrap();
    let map_elem = doc.descendants().find(|n| n.has_tag_name("map")).unwrap();

    let nodes_elem = map_elem
        .children()
        .find(|n| n.has_tag_name("nodes"))
        .unwrap();
    let arcs_elem = map_elem
        .children()
        .find(|n| n.has_tag_name("arcs"))
        .unwrap();

    let nodes = nodes_elem
        .children()
        .filter(|n| n.has_tag_name("node"))
        .map(|n| {
            let id = n.attribute("id").unwrap().parse::<String>().unwrap();
            let lat = n.attribute("latitude").unwrap().parse::<f64>().unwrap() / 100000.0;
            let long = n.attribute("longitude").unwrap().parse::<f64>().unwrap() / 100000.0;

            let position_on_screen = TRANSLATOR.lock().project(lat, long);

            Node::new(id, position_on_screen)
        })
        .collect::<Vec<Node>>();

    let edges = arcs_elem
        .children()
        .filter(|n| n.has_tag_name("arc"))
        .map(|n| {
            let from = n.attribute("from").unwrap().parse::<String>().unwrap();
            let to = n.attribute("to").unwrap().parse::<String>().unwrap();
            let length = n.attribute("length").unwrap().parse::<f32>().unwrap();

            let from_node = nodes.iter().find(|n| n.id == from).unwrap().clone();
            let to_node = nodes.iter().find(|n| n.id == to).unwrap().clone();

            Edge::new(from_node, to_node, length)
        })
        .collect::<Vec<Edge>>();

    let mut neighboors: HashMap<Node, Vec<Edge>> = HashMap::new();
    edges.iter().for_each(|edge| {
        neighboors
            .entry(edge.from.clone())
            .and_modify(|neighboors| neighboors.push(edge.clone()))
            .or_insert(vec![edge.clone()]);
    });

    (nodes, edges, neighboors)
}
