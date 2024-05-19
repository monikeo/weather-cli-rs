use crate::Request::*;
use crate::Weather::WeatherResponse::*;
use anyhow::Result;
use serde_json;

pub async fn fetch_weather(
    /*api_key: &str, city: &str, lang: &str*/ url: &str,
) -> Result<WeatherResponse> {
    /*
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&lang={}&appid={}",
        "Phnom Penh",
        "en",
        Api::other_api
    );
    */
    let response = get_request(&url).await;

    let response_json: WeatherResponse = serde_json::from_str(&response?)?;
    Ok(response_json)
}
