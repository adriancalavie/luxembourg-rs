use crate::{arc::Arc, node::Node, translator::Translator};

pub fn parse_xml(file_name: &str, mut translator: Translator) -> (Vec<Node>, Vec<Arc>) {
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

            let position_on_screen = translator.project(lat, long);

            Node::new(id, position_on_screen)
        })
        .collect::<Vec<Node>>();

    let arcs = arcs_elem
        .children()
        .filter(|n| n.has_tag_name("arc"))
        .map(|n| {
            let from = n.attribute("from").unwrap().parse::<String>().unwrap();
            let to = n.attribute("to").unwrap().parse::<String>().unwrap();
            let length = n.attribute("length").unwrap().parse::<f32>().unwrap();

            let from_node = nodes.iter().find(|n| n.id == from).unwrap().clone();
            let to_node = nodes.iter().find(|n| n.id == to).unwrap().clone();

            let from_position = from_node.position;
            let to_position = to_node.position;

            Arc::new(from_position, to_position, length)
        })
        .collect::<Vec<Arc>>();

    (nodes, arcs)
}
