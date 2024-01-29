pub trait Is2D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl Is2D for egui::Vec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Is2D for egui::Pos2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}
