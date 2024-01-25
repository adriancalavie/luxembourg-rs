use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub longitude: f64,
    pub latitude: f64,
}

impl Node {
    pub fn new(id: String, longitude: f64, latitude: f64) -> Self {
        Self {
            id,
            longitude,
            latitude,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[id: {}][long: {}][lat: {}]",
            self.id, self.longitude, self.latitude
        )
    }
}

pub fn get_some_nodes() -> Vec<Node> {
    vec![
        Node::new("1".to_string(), 4963454.0 / 100000.0, 621476.0 / 100000.0),
        Node::new("2".to_string(), 4959493.0 / 100000.0, 614350.0 / 100000.0),
        Node::new("3".to_string(), 4959247.0 / 100000.0, 612096.0 / 100000.0),
        Node::new("4".to_string(), 4959206.0 / 100000.0, 612162.0 / 100000.0),
    ]
}
