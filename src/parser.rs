use crate::{arc::Arc, node::Node};

pub fn parse_xml(file_name: &str) -> (Vec<Node>, Vec<Arc>) {
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

            Node::new(id, lat, long)
        })
        .collect::<Vec<Node>>();

    let arcs = arcs_elem
        .children()
        .filter(|n| n.has_tag_name("arc"))
        .map(|n| {
            let from = n.attribute("from").unwrap().parse::<String>().unwrap();
            let to = n.attribute("to").unwrap().parse::<String>().unwrap();
            let length = n.attribute("length").unwrap().parse::<f64>().unwrap();

            let from_node = nodes.iter().find(|n| n.id == from).unwrap().clone();
            let to_node = nodes.iter().find(|n| n.id == to).unwrap().clone();

            Arc::new(
                from_node.latitude,
                from_node.longitude,
                to_node.latitude,
                to_node.longitude,
                length,
            )
        })
        .collect::<Vec<Arc>>();

    (nodes, arcs)
}
