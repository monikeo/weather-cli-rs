use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CloudsData {
    all: u32,
}

impl CloudsData {
    pub fn new(all: u32) -> Self {
        Self { all }
    }
    pub fn all(&self) -> u32 {
        self.all
    }
}
