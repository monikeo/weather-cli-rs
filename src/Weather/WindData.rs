use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WindData {
    speed: f32,
    deg: u32,
    gust: Option<f64>,
}

impl WindData {
    pub fn new(speed: f32, deg: u32) -> Self {
        Self {
            speed,
            deg,
            gust: None,
        }
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn deg(&self) -> u32 {
        self.deg
    }
}
