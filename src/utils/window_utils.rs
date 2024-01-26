use log::debug;

#[derive(Debug, Clone)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        debug!("screen width: {}, screen height: {}", width, height);
        Self { width, height }
    }
}
