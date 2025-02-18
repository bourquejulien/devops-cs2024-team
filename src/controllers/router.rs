use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Json};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::door_handler::ParallelExecutor;
use crate::map::handle_map;
use crate::weather::{get_weather, Location};

#[derive(Deserialize)]
pub struct Route {
    request: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Door {
    hash: String,
}

pub async fn post(State(executor): State<Arc<Mutex<ParallelExecutor>>>, route: Query<Route>, body: Option<String>) -> (StatusCode, Response) {
    if let Some(request) = &route.request {
        return match request.as_str() {
            "status" => (StatusCode::OK, "OK".into_response()),
            "weather" => handle_weather(body.as_ref()),
            "map" => handle_map(body.as_ref()),
            "door" => decode_password(body.as_ref(), executor),
            &_ => (StatusCode::NOT_FOUND, String::from("Invalid request provided").into_response())
        };
    }

    (StatusCode::NOT_FOUND, String::from("No request provided").into_response())
}

fn decode_password(payload: Option<&String>, executor: Arc<Mutex<ParallelExecutor>>) -> (StatusCode, Response)  {
    let door: Result<Door, String> = payload
        .ok_or(String::from("No payload"))
        .and_then(|payload| serde_json::from_str(payload.as_str()).map_err(|_| String::from("Failed to parse payload")));

    match door {
        Ok(door) => {
            if let Ok(executor) = executor.lock() {
                executor.execute(door.hash);
                return (StatusCode::OK, "Request queued".into_response());
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to acquire lock".into_response());
        }
        Err(error) => {(StatusCode::BAD_REQUEST, error.into_response())}
    }
}

fn handle_weather(payload: Option<&String>) -> (StatusCode, Response) {
    let result: Result<Location, String> = payload
        .ok_or(String::from("No payload"))
        .and_then(|payload| serde_json::from_str(payload.as_str()).map_err(|_| String::from("Failed to parse payload")));

    if let Err(err) = result {
        return (StatusCode::BAD_REQUEST, err.into_response())
    }

    let location = result.unwrap();

    if let Some(weather) = get_weather(&location) {
        return (StatusCode::OK, Json(weather).into_response())
    }

    return (StatusCode::BAD_REQUEST, "Cannot fetch weather".into_response())
}
