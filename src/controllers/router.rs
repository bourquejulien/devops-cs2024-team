use std::fmt::Debug;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::map::handle_map;

#[derive(Deserialize)]
pub struct Route {
    request: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    location: String,
    temperature: f64,
    #[allow(non_snake_case)]
    windSpeed: f64,
    precipitation: f64,
    description: String
}

pub async fn post(route: Query<Route>, body: Option<String>) -> (StatusCode, Response) {
    if let Some(request) = &route.request {
        return match request.as_str() {
            "status" => (StatusCode::OK, "OK".into_response()),
            "weather" => (StatusCode::OK, handle_weather()),
            "map" => handle_map(body.as_ref()),
            &_ => (StatusCode::NOT_FOUND, String::from("Invalid request provided").into_response())
        };
    }

    // let response = ureq::get("http://ai.private.dev.cs2024.one/jungle").call();

    // return match response {
    //     Ok(data) => {
    //         if let Ok(str_data) = data.into_string() {
    //             return (StatusCode::OK, Html(format!("<h1>Received: {str_data}<h1/>", )));
    //         }
    //         (StatusCode::INTERNAL_SERVER_ERROR, Html("<h1>Call failed!!<h1/>".parse().unwrap()))
    //     }
    //
    //     Err(err) => {
    //         (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("<h1>Call failed with reason: <h1/>\n{}", err)))
    //     }
    // }

    (StatusCode::NOT_FOUND, String::from("No request provided").into_response())
}

fn handle_weather() -> Response {
    Json(Weather{
        location: "test".parse().unwrap(),
        temperature: 1.,
        windSpeed: 1.,
        precipitation: 1.,
        description: "test".parse().unwrap()}).into_response()
}
