use egui::Pos2;

pub fn euclidean_distance(a: &Pos2, b: &Pos2) -> f32 {
    (*a - *b).length()
}

pub fn manhattan_distance(a: &Pos2, b: &Pos2) -> f32 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    dx + dy
}
