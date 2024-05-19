use std::collections::HashMap;

const WEATHER_ICONS: &[(&str, &str)] = &[
    // Day icons
    ("01d", "☀️"),
    ("02d", "⛅️"),
    ("03d", "☁️"),
    ("04d", "☁️"),
    ("09d", "🌧"),
    ("10d", "🌦"),
    ("11d", "⛈"),
    ("13d", "🌨"),
    ("50d", "🌫"),
    // Night icons
    ("01n", "🌙"),
    ("02n", "☁️"),
    ("03n", "☁️"),
    ("04n", "☁️"),
    ("09n", "🌧"),
    ("10n", "🌦"),
    ("11n", "⛈"),
    ("13n", "🌨"),
    ("50n", "🌫"),
];

pub fn get_weather_icon(weather_code: &str) -> Option<&str> {
    let weather_icons: HashMap<&str, &str> = WEATHER_ICONS.iter().cloned().collect();
    let icon = weather_icons.get(weather_code);
    if !icon.is_none() {
        Some(icon.unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_work_weather_icon() {
        let icon = get_weather_icon("09n");
        println!("{:?}", icon);
    }
}
