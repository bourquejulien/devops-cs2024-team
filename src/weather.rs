use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    temperature: f64,
    #[allow(non_snake_case)]
    windSpeed: f64,
    precipitation: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    x: f64,
    y: f64,
}

pub(crate) fn get_weather(location: &Location) -> Option<Weather> {
    let result = ureq::get(format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,precipitation,wind_speed_10m", location.x, location.y).as_str())
        .call().ok()?;

    let result = result.into_string().ok()?;
    let json: Value = serde_json::from_str(result.as_str()).ok()?;

    return Some(Weather{
        temperature: json["current"]["temperature_2m"].as_f64()?,
        windSpeed: json["current"]["wind_speed_10m"].as_f64()?,
        precipitation: json["current"]["precipitation"].as_f64()?
    });
}
