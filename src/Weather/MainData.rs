use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MainData {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: u32,
    humidity: u32,
}

impl MainData {
    pub fn new(
        temp: f32,
        feels_like: f32,
        temp_min: f32,
        temp_max: f32,
        pressure: u32,
        humidity: u32,
    ) -> Self {
        Self {
            temp,
            feels_like,
            temp_min,
            temp_max,
            pressure,
            humidity,
        }
    }

    pub fn temp(&self) -> f32 {
        self.temp
    }
    pub fn feels_like(&self) -> f32 {
        self.feels_like
    }
    pub fn temp_min(&self) -> f32 {
        self.temp_min
    }
    pub fn temp_max(&self) -> f32 {
        self.temp_max
    }
    pub fn pressure(&self) -> u32 {
        self.pressure
    }
    pub fn humidity(&self) -> u32 {
        self.humidity
    }
}
