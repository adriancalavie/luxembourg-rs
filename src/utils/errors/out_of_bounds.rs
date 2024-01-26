use crate::utils::WindowSize;

#[derive(Debug)]
pub struct OutOfBoundsError {
    position: egui::Pos2,
    screen_size: WindowSize,
}

impl OutOfBoundsError {
    pub fn new(position: egui::Pos2, screen_size: WindowSize) -> Self {
        Self {
            position,
            screen_size,
        }
    }
}

impl std::fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Position {:?} is out of screen bounds {:?}",
            self.position, self.screen_size
        )
    }
}

impl std::error::Error for OutOfBoundsError {}
