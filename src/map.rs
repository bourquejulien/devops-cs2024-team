use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize};

#[derive(Deserialize)]
struct Request {
    x: f64,
    y: f64,
    size: u8,
}

pub fn handle_map(payload: Option<&String>) -> (StatusCode, Response) {
    let request: Result<Request, String> = payload
        .ok_or(String::from("No payload"))
        .and_then(|payload| serde_json::from_str(payload.as_str()).map_err(|err| String::from("Failed to parse payload")));

    if let Ok(data) = request {
        let result = ureq::get(format!("http://localhost:7000/?x={}&y={}&size={}", data.x, data.y, data.size).as_str()).call();

        if let Ok(map) = result {
            return (StatusCode::OK, map.into_string().unwrap().into_response());
        }

        return (StatusCode::INTERNAL_SERVER_ERROR, "Request failed".into_response())
    }

    return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse payload".into_response())
}
