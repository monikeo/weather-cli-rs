use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SysData {
    #[serde(rename = "type")]
    typ: Option<u32>,
    id: Option<u32>,
    country: String,
    sunrise: u32,
    sunset: u32,
}

impl SysData {
    pub fn new(typ: u32, id: u32, country: String, sunrise: u32, sunset: u32) -> Self {
        Self {
            typ: Some(typ),
            id: Some(id),
            country,
            sunrise,
            sunset,
        }
    }
    pub fn typ(&self) -> Option<u32> {
        self.typ
    }
    pub fn id(&self) -> Option<u32> {
        self.id
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn sunrise(&self) -> u32 {
        self.sunrise
    }
    pub fn sunset(&self) -> u32 {
        self.sunset
    }
}
