use std::collections::HashMap;

use crate::{
    models::{Edge, Node},
    translator::TRANSLATOR,
};

pub fn parse_xml(file_name: &str) -> (Vec<Node>, Vec<Edge>, HashMap<Node, Vec<Node>>) {
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

    let mut neighboors: HashMap<Node, Vec<Node>> = HashMap::new();
    let edges = arcs_elem
        .children()
        .filter(|n| n.has_tag_name("arc"))
        .map(|n| {
            let from = n.attribute("from").unwrap().parse::<String>().unwrap();
            let to = n.attribute("to").unwrap().parse::<String>().unwrap();
            let length = n.attribute("length").unwrap().parse::<f32>().unwrap();

            let from_node = nodes.iter().find(|n| n.id == from).unwrap().clone();
            let to_node = nodes.iter().find(|n| n.id == to).unwrap().clone();

            neighboors
                .entry(from_node.clone())
                .and_modify(|neighboors| neighboors.push(to_node.clone()))
                .or_insert(vec![to_node.clone()]);
            
            neighboors
                .entry(to_node.clone())
                .and_modify(|neighboors| neighboors.push(from_node.clone()))
                .or_insert(vec![from_node.clone()]);

            let from_position = from_node.position;
            let to_position = to_node.position;

            Edge::new(from_position, to_position, length)
        })
        .collect::<Vec<Edge>>();

    (nodes, edges, neighboors)
}
