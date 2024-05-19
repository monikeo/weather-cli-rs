use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CoordinateData {
    #[serde(rename = "lon")]
    longitude: f32,
    #[serde(rename = "lat")]
    latitude: f32,
}

impl CoordinateData {
    pub fn new(longitude: f32, latitude: f32) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
    pub fn lon(&self) -> f32 {
        self.longitude
    }
    pub fn lat(&self) -> f32 {
        self.latitude
    }
}
