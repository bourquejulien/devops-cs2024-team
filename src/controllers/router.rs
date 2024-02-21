use std::fmt::Debug;
use std::sync::Mutex;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::door_handler::ParallelExecutor;
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

#[derive(Debug, Serialize, Deserialize)]
struct Door {
    hash: String,
}

lazy_static! {
    static ref EXECUTOR: Mutex<ParallelExecutor> = Mutex::new(ParallelExecutor::new());
}

pub async fn post(route: Query<Route>, body: Option<String>) -> (StatusCode, Response) {
    if let Some(request) = &route.request {
        return match request.as_str() {
            "status" => (StatusCode::OK, "OK".into_response()),
            "weather" => (StatusCode::OK, handle_weather()),
            "map" => handle_map(body.as_ref()),
            "door" => decode_password(body.as_ref()),
            &_ => (StatusCode::NOT_FOUND, String::from("Invalid request provided").into_response())
        };
    }

    (StatusCode::NOT_FOUND, String::from("No request provided").into_response())
}

fn decode_password(payload: Option<&String>) -> (StatusCode, Response)  {
    let door: Result<Door, String> = payload
        .ok_or(String::from("No payload"))
        .and_then(|payload| serde_json::from_str(payload.as_str()).map_err(|err| String::from("Failed to parse payload")));

    match door {
        Ok(door) => {
            if let Ok(executor) = EXECUTOR.lock() {
                executor.execute(door.hash);
                return (StatusCode::OK, "Request queued".into_response());
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to acquire lock".into_response());
        }
        Err(error) => {(StatusCode::BAD_REQUEST, error.into_response())}
    }
}

fn handle_weather() -> Response {
    Json(Weather{
        location: "test".parse().unwrap(),
        temperature: 1.,
        windSpeed: 1.,
        precipitation: 1.,
        description: "test".parse().unwrap()}).into_response()
}
