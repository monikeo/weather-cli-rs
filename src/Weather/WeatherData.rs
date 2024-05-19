use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

impl WeatherData {
    pub fn new(id: u32, main: String, description: String, icon: String) -> Self {
        Self {
            id,
            main,
            description,
            icon,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn main(&self) -> &str {
        &self.main
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn icon(&self) -> &str {
        &self.icon
    }
}
