pub const other_api: &'static str = "e5c9e66275afa5767ef301b14caaca58";

#[derive(Debug)]
pub enum Mode {
    Xml,
    Html,
    Json,
}

impl Mode {
    pub fn get(&self) -> &'static str {
        match &self {
            Self::Xml => "xml",
            Self::Html => "html",
            Self::Json => "json",
        }
    }
}
#[derive(Debug)]
pub enum Units {
    // Kelvin is default and as standard
    Standard,
    // Celsius use metric
    Metric,
    // Fahrenheit use imperial
    Imperial,
}
impl Units {
    pub fn get(&self) -> &'static str {
        match &self {
            Self::Standard => "standard",
            Self::Metric => "metric",
            Self::Imperial => "imperial",
        }
    }
    pub fn get_prefix(&self) -> &'static str {
        match &self {
            Self::Standard => "K",
            Self::Metric => "°C",
            Self::Imperial => "°F",
        }
    }
}

#[derive(Debug)]
pub enum Lang {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Vietnamese,
    Thai,
    Korean,
}

impl Lang {
    pub fn get(&self) -> &'static str {
        match &self {
            Self::English => "en",
            Self::ChineseSimplified => "zh_cn",
            Self::ChineseTraditional => "zh_tw",
            Self::Vietnamese => "vi",
            Self::Thai => "th",
            Self::Korean => "kr",
        }
    }
}

#[derive(Debug)]
pub struct OpenWeatherApiUrl {
    q: String,
    appid: String,
    mode: Option<Mode>,
    units: Option<Units>,
    lang: Option<Lang>,
}

impl OpenWeatherApiUrl {
    pub fn new(q: &str, appid: &str) -> Self {
        Self {
            q: q.trim().to_string(),
            appid: appid.trim().to_string(),
            mode: None,
            units: Some(Units::Metric),
            lang: None,
        }
    }
    pub fn mode(&self) -> &Option<Mode> {
        &self.mode
    }
    pub fn units(&self) -> Option<&Units> {
        /*
        if self.units.is_none() {
            None
        } else {
            Some(&self.units.as_ref().unwrap())
        }
        */
        return self.units.as_ref();
    }
    pub fn lang(&self) -> &Option<Lang> {
        &self.lang
    }
    pub fn set_city(&mut self, q: &str) {
        self.q = q.to_string();
    }
    pub fn set_api(&mut self, appid: &str) {
        self.appid = appid.to_string();
    }
    pub fn set_mode(&mut self, mode: Option<Mode>) {
        self.mode = mode;
    }
    pub fn set_units(&mut self, units: Option<Units>) {
        self.units = units;
    }
    pub fn set_lang(&mut self, lang: Option<Lang>) {
        self.lang = lang;
    }

    pub fn get_url(&self) -> String {
        let mut url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            &self.q, &self.appid
        );
        if !self.mode.is_none() {
            url.push_str(format!("&mode={}", self.mode.as_ref().unwrap().get()).as_str());
        }
        if !self.units.is_none() {
            url.push_str(format!("&units={}", self.units.as_ref().unwrap().get()).as_str());
        }
        if !self.lang.is_none() {
            url.push_str(format!("&lang={}", self.lang.as_ref().unwrap().get()).as_str());
        }
        println!("{}", url);
        url
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_work() {
        let weather_url = OpenWeatherApiUrl::new("Phnom Penh", other_api);
        println!("{}", weather_url.get_url());
    }
}
