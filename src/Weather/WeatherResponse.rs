use crate::Weather::{
    CloudsData::*, CoordinateData::*, MainData::*, SysData::*, WeatherData::*, WindData::*,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    coord: CoordinateData,
    weather: Vec<WeatherData>,
    base: String,
    main: MainData,
    visibility: u32,
    wind: WindData,
    clouds: CloudsData,
    dt: u32,
    sys: SysData,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

impl WeatherResponse {
    pub fn coord(&self) -> &CoordinateData {
        &self.coord
    }
    pub fn weather(&self) -> Vec<&WeatherData> {
        vec![&self.weather[0]]
    }
    pub fn main(&self) -> &MainData {
        &self.main
    }
    pub fn base(&self) -> &str {
        &self.base
    }
    pub fn wind(&self) -> &WindData {
        &self.wind
    }
    pub fn clouds(&self) -> &CloudsData {
        &self.clouds
    }
    pub fn visibility(&self) -> u32 {
        self.visibility
    }
    pub fn dt(&self) -> u32 {
        self.dt
    }
    pub fn sys(&self) -> &SysData {
        &self.sys
    }
    pub fn timezone(&self) -> i32 {
        self.timezone
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn cod(&self) -> u32 {
        self.cod
    }
}
